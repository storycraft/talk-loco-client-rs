/*
 * Created on Wed Jul 28 2021
 *
 * Copyright (c) storycraft. Licensed under the MIT Licence.
 */

use std::{
    collections::{BTreeMap, BTreeSet, VecDeque},
    sync::{Arc, Mutex},
};

use bson::Document;
use futures::{AsyncRead, AsyncWrite};
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

/// Async Command session with command cache.
/// Provide methods for requesting command response and broadcast command handling.
/// Useful when creating client.
#[derive(Debug, Clone)]
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

    /// Try unwrapping stream.
    /// If any request not finished exist this method will fail return Err.
    pub fn try_unwrap(self) -> Result<BsonCommandManager<S>, ()> {
        match Arc::try_unwrap(self.inner) {
            Ok(inner) => Ok(inner.into_inner().unwrap().manager),
            Err(_) => Err(()),
        }
    }
}

impl<S: AsyncWrite + AsyncRead + Unpin> BsonCommandSession<S> {
    /// Request given command.
    /// The response is guaranteed to have same id of request command.
    pub async fn request<T: Serialize>(
        &self,
        command: &BsonCommand<T>,
    ) -> Result<Request<S>, WriteError> {
        let mut inner = self.inner.lock().unwrap();

        let request_id = inner.manager.write_async(command).await?;

        inner.request_set.insert(request_id);

        Ok(Request {
            request_id,

            inner: self.inner.clone(),
        })
    }
}

impl<S: AsyncRead + Unpin> BsonCommandSession<S> {
    /// Read incoming broadcast commands
    pub async fn next_broadcast(&self) -> Result<(i32, BsonCommand<Document>), ReadError> {
        let inner = self.inner.clone();

        loop {
            let mut inner = inner.lock().unwrap();

            if let Some(unread) = inner.broadcasts.pop_front() {
                return Ok(unread);
            }

            inner.read().await?;
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

impl<S: AsyncRead + Unpin> BsonCommandSessionInner<S> {
    /// Read first [BsonCommand] incoming.
    pub async fn read(&mut self) -> Result<(), ReadError> {
        let (id, read) = self.manager.read_async().await?;

        if self.request_set.remove(&id) {
            self.response_map.insert(id, read);
        } else {
            self.broadcasts.push_back((id, read));
        }

        Ok(())
    }

    /// Read specific [BsonCommand] added in request_set
    pub async fn read_id(&mut self, id: i32) -> Result<BsonCommand<Document>, ReadError> {
        if let Some(res) = self.response_map.remove(&id) {
            Ok(res)
        } else {
            self.request_set.insert(id);

            loop {
                self.read().await?;

                if let Some(res) = self.response_map.remove(&id) {
                    return Ok(res);
                }
            }
        }
    }
}

/// BsonCommand request
pub struct Request<S> {
    request_id: i32,

    inner: Arc<Mutex<BsonCommandSessionInner<S>>>,
}

impl<S> Request<S> {
    pub fn request_id(&self) -> i32 {
        self.request_id
    }
}

impl<S: AsyncRead + Unpin> Request<S> {
    pub async fn response(self) -> Result<BsonCommand<Document>, ReadError> {
        let mut inner = self.inner.lock().unwrap();

        inner.read_id(self.request_id).await
    }
}

impl<S> Drop for Request<S> {
    fn drop(&mut self) {
        let mut inner = self.inner.lock().unwrap();

        if !inner.request_set.remove(&self.request_id) {
            inner.response_map.remove(&self.request_id);
        }
    }
}
