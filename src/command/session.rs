/*
 * Created on Wed Jul 28 2021
 *
 * Copyright (c) storycraft. Licensed under the MIT Licence.
 */

use std::{
    collections::BTreeMap,
    io::{Read, Write},
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

impl<S: Write> BsonCommandSession<S> {
    /// Send and create response ticket of this request.
    /// The response is guaranteed to have same id of request command.
    pub fn request<T: Serialize>(
        &mut self,
        command: &BsonCommand<T>,
    ) -> Result<i32, WriteError> {
        self.manager.write(command)
    }
}

impl<S: AsyncWrite + Unpin> BsonCommandSession<S> {
    /// Send and create response ticket of this request asynchronously.
    /// The response is guaranteed to have same id returned.
    pub async fn request_async<T: Serialize>(
        &mut self,
        command: &BsonCommand<T>,
    ) -> Result<i32, WriteError> {
        self.manager.write_async(command).await
    }
}

impl<S: Read> BsonCommandSession<S> {
    /// Read next [BsonCommand]
    pub fn read(&mut self) -> Result<(i32, BsonCommand<Document>), ReadError> {
        if let Some(next_id) = self.read_map.keys().next().copied() {
            Ok((next_id, self.read_map.remove(&next_id).unwrap()))
        } else {
            let read = self.manager.read()?;
            Ok(read)
        }
    }

    /// Read [BsonCommand] response
    pub fn response(&mut self, id: i32) -> Result<BsonCommand<Document>, ReadError> {
        if let Some(read) = self.read_map.remove(&id) {
            return Ok(read);
        }

        loop {
            let (read_id, read) = self.manager.read()?;

            if read_id == id {
                return Ok(read);
            } else {
                self.read_map.insert(id, read);
            }
        }
    }
}

impl<S: AsyncRead + Unpin> BsonCommandSession<S> {
    /// Read next [BsonCommand] asynchronously
    pub async fn read_async(&mut self) -> Result<(i32, BsonCommand<Document>), ReadError> {
        if let Some(next_id) = self.read_map.keys().next().copied() {
            Ok((next_id, self.read_map.remove(&next_id).unwrap()))
        } else {
            let read = self.manager.read_async().await?;
            Ok(read)
        }
    }

    /// Read [BsonCommand] response asynchronously
    pub async fn response_async(&mut self, id: i32) -> Result<BsonCommand<Document>, ReadError> {
        if let Some(read) = self.read_map.remove(&id) {
            return Ok(read);
        }

        loop {
            let (read_id, read) = self.manager.read_async().await?;

            if read_id == id {
                return Ok(read);
            } else {
                self.read_map.insert(id, read);
            }
        }
    }
}
