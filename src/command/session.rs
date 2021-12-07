/*
 * Created on Wed Jul 28 2021
 *
 * Copyright (c) storycraft. Licensed under the MIT Licence.
 */

use std::collections::BTreeMap;

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
#[derive(Debug)]
pub struct BsonCommandSession<S> {
    read_map: BTreeMap<i32, BsonCommand<Document>>,

    manager: BsonCommandManager<S>,
}

impl<S> BsonCommandSession<S> {
    /// Create new [BsonCommandSession]
    pub fn new(manager: BsonCommandManager<S>) -> Self {
        Self {
            read_map: BTreeMap::new(),
            manager,
        }
    }

    /// Consume self and returns inner [BsonCommandManager]
    pub fn into_inner(self) -> BsonCommandManager<S> {
        self.manager
    }
}

impl<S: AsyncWrite + AsyncRead + Unpin> BsonCommandSession<S> {
    /// Request given command.
    /// The response is guaranteed to have same id of request command.
    pub async fn request<T: Serialize>(
        &mut self,
        command: &BsonCommand<T>,
    ) -> Result<Request, WriteError> {
        let request_id = self.manager.write_async(command).await?;

        Ok(Request(request_id))
    }
}

impl<S: AsyncRead + Unpin> BsonCommandSession<S> {
    /// Read next [BsonCommand]
    pub async fn read(&mut self) -> Result<(i32, BsonCommand<Document>), ReadError> {
        if let Some(next_id) = self.read_map.keys().next().copied() {
            Ok((next_id, self.read_map.remove(&next_id).unwrap()))
        } else {
            let read = self.manager.read_async().await?;
            Ok(read)
        }
    }

    /// Read [BsonCommand] with specific id
    pub async fn read_id(&mut self, id: i32) -> Result<BsonCommand<Document>, ReadError> {
        loop {
            if let Some(res) = self.read_map.remove(&id) {
                return Ok(res);
            }

            let (id, read) = self.manager.read_async().await?;
            self.read_map.insert(id, read);
        }
    }
}

/// Pending [BsonCommand] request
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Request(i32);

impl Request {
    pub fn request_id(&self) -> i32 {
        self.0
    }
}

impl Request {
    pub async fn response<S: AsyncRead + Unpin>(
        self,
        session: &mut BsonCommandSession<S>,
    ) -> Result<BsonCommand<Document>, ReadError> {
        session.read_id(self.0).await
    }
}
