/*
 * Created on Wed Jul 28 2021
 *
 * Copyright (c) storycraft. Licensed under the MIT Licence.
 */

use std::{
    collections::{BTreeMap, BTreeSet, VecDeque},
    future::Future,
    io::{self, Read, Write},
    pin::Pin,
    sync::{Arc, Mutex, Weak},
    task::{Context, Poll},
};

use bson::Document;
use futures::Stream;
use loco_protocol::command::codec::StreamError;
use serde::Serialize;

use super::{
    manager::{BsonCommandManager, ReadError, WriteError},
    BsonCommand,
};

#[derive(Debug)]
pub enum RequestError {
    Write(WriteError),
    Read(ReadError),
}

impl From<WriteError> for RequestError {
    fn from(err: WriteError) -> Self {
        Self::Write(err)
    }
}

impl From<ReadError> for RequestError {
    fn from(err: ReadError) -> Self {
        Self::Read(err)
    }
}

/// Command session with command cache.
/// Provide methods for requesting command response and broadcast command handling.
/// Useful when creating client.
/// Using non blocking mode highly recommended.
#[derive(Debug)]
pub struct BsonCommandSession<S> {
    inner: Arc<Mutex<BsonCommandSessionInner<S>>>,
}

impl<S> BsonCommandSession<S> {
    /// Create new [BsonCommandSession]
    pub fn new(manager: BsonCommandManager<S>) -> Self {
        Self {
            inner: Arc::new(Mutex::new(BsonCommandSessionInner {
                request_set: BTreeSet::new(),
                response_map: BTreeMap::new(),
                broadcasts: VecDeque::new(),
                manager,
            })),
        }
    }

    /// Try unwrapping [BsonCommandManager].
    /// If any request not finished exist this method will fail return Err.
    pub fn try_unwrap(self) -> Result<BsonCommandManager<S>, ()> {
        match Arc::try_unwrap(self.inner) {
            Ok(inner) => Ok(inner.into_inner().unwrap().manager),
            Err(_) => Err(()),
        }
    }
}

impl<S: Read + Write> BsonCommandSession<S> {
    /// Request response for given command.
    /// The response is guaranteed to have same id of request command.
    pub fn request<T: Serialize>(
        &self,
        command: &BsonCommand<T>,
    ) -> Result<ResponseFuture<S>, RequestError> {
        let request_id = self.inner.lock().unwrap().request(command)?;

        Ok(ResponseFuture {
            request_id,
            inner: self.inner.clone(),
        })
    }
}

impl<S: Read> BsonCommandSession<S> {
    /// Read incoming broadcast commands
    pub fn broadcasts(&self) -> BroadcastStream<S> {
        BroadcastStream {
            inner: Arc::downgrade(&self.inner),
        }
    }
}

#[derive(Debug)]
struct BsonCommandSessionInner<S> {
    request_set: BTreeSet<i32>,
    response_map: BTreeMap<i32, BsonCommand<Document>>,

    broadcasts: VecDeque<(i32, BsonCommand<Document>)>,

    manager: BsonCommandManager<S>,
}

impl<S: Read + Write> BsonCommandSessionInner<S> {
    pub fn request<T: Serialize>(&mut self, command: &BsonCommand<T>) -> Result<i32, RequestError> {
        let request_id = self.manager.write(command)?;
        Ok(request_id)
    }
}

impl<S: Read> BsonCommandSessionInner<S> {
    /// Poll first [BsonCommand] incoming.
    pub fn poll(&mut self) -> Poll<Result<(), ReadError>> {
        match self.manager.read() {
            Ok((id, read)) => {
                if self.request_set.remove(&id) {
                    self.response_map.insert(id, read);
                } else {
                    self.broadcasts.push_back((id, read));
                }

                Poll::Ready(Ok(()))
            }

            Err(ReadError::Codec(StreamError::Io(err)))
                if err.kind() == io::ErrorKind::WouldBlock =>
            {
                Poll::Pending
            }

            Err(err) => Poll::Ready(Err(err)),
        }
    }

    /// Poll specific [BsonCommand]
    pub fn poll_id(&mut self, id: i32) -> Poll<Result<BsonCommand<Document>, ReadError>> {
        if let Some(res) = self.response_map.remove(&id) {
            Poll::Ready(Ok(res))
        } else {
            self.request_set.insert(id);

            match self.poll() {
                Poll::Ready(result) => match result {
                    Ok(_) => {
                        if let Some(res) = self.response_map.remove(&id) {
                            Poll::Ready(Ok(res))
                        } else {
                            Poll::Pending
                        }
                    }
                    Err(err) => Poll::Ready(Err(err)),
                },
                Poll::Pending => Poll::Pending,
            }
        }
    }
}

/// Future for BsonCommand response.
/// Request response must be processed before requesting other command.
#[must_use = "futures do nothing unless polled"]
pub struct ResponseFuture<S> {
    request_id: i32,
    inner: Arc<Mutex<BsonCommandSessionInner<S>>>,
}

impl<S> ResponseFuture<S> {
    pub fn request_id(&self) -> i32 {
        self.request_id
    }
}

impl<S: Read> Future for ResponseFuture<S> {
    type Output = Result<BsonCommand<Document>, ReadError>;

    fn poll(self: Pin<&mut Self>, ctx: &mut Context<'_>) -> Poll<Self::Output> {
        let mut inner = self.inner.lock().unwrap();

        match inner.poll_id(self.request_id) {
            Poll::Ready(res) => Poll::Ready(res),

            Poll::Pending => {
                ctx.waker().wake_by_ref();
                Poll::Pending
            }
        }
    }
}

pub struct BroadcastStream<S> {
    inner: Weak<Mutex<BsonCommandSessionInner<S>>>,
}

impl<S: Read> Stream for BroadcastStream<S> {
    type Item = Result<(i32, BsonCommand<Document>), ReadError>;

    fn poll_next(self: Pin<&mut Self>, ctx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        match self.inner.upgrade() {
            Some(inner) => {
                let mut inner = inner.lock().unwrap();

                if let Some(unread) = inner.broadcasts.pop_front() {
                    Poll::Ready(Some(Ok(unread)))
                } else {
                    match inner.poll() {
                        Poll::Ready(res) => {
                            if let Err(err) = res {
                                Poll::Ready(Some(Err(err)))
                            } else {
                                ctx.waker().wake_by_ref();
                                Poll::Pending
                            }
                        }

                        Poll::Pending => {
                            ctx.waker().wake_by_ref();
                            Poll::Pending
                        }
                    }
                }
            }
            None => return Poll::Ready(None),
        }
    }
}
