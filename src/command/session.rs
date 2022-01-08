/*
 * Created on Wed Jul 28 2021
 *
 * Copyright (c) storycraft. Licensed under the MIT Licence.
 */

use std::{
    io::{Read, Write},
};

use bson::Document;
use futures::{AsyncRead, AsyncWrite};
use indexmap::IndexMap;
use loco_protocol::command::codec::StreamError;
use serde::Serialize;

use super::{
    codec::{BsonCommandCodec, ReadError, WriteError},
    BsonCommand, ReadBsonCommand,
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

/// Async Command session.
/// Provide methods for requesting command response and broadcast command handling.
/// Useful when creating client.
#[derive(Debug)]
pub struct BsonCommandSession<S> {
    current_id: i32,
    read_map: IndexMap<i32, BsonCommand<Document>>,

    codec: BsonCommandCodec<S>,
}

impl<S> BsonCommandSession<S> {
    /// Create new [BsonCommandSession]
    pub fn new(stream: S) -> Self {
        Self {
            current_id: 0,
            read_map: IndexMap::new(),

            codec: BsonCommandCodec::new(stream),
        }
    }

    pub fn current_id(&self) -> i32 {
        self.current_id
    }

    /// Consume self and returns inner stream
    pub fn into_inner(self) -> S {
        self.codec.into_inner()
    }
}

impl<S: Write> BsonCommandSession<S> {
    /// Send and create response ticket of this request.
    /// The response is guaranteed to have same id of request command.
    pub fn request(&mut self, command: &BsonCommand<impl Serialize>) -> Result<i32, WriteError> {
        let request_id = self.current_id;
        self.current_id += 1;

        self.codec.write(request_id, command)?;
        self.codec
            .flush()
            .map_err(|err| WriteError::Codec(StreamError::Io(err)))?;

        Ok(request_id)
    }
}

impl<S: AsyncWrite + Unpin> BsonCommandSession<S> {
    /// Send and create response ticket of this request asynchronously.
    /// The response is guaranteed to have same id returned.
    pub async fn request_async(
        &mut self,
        command: &BsonCommand<impl Serialize>,
    ) -> Result<i32, WriteError> {
        let request_id = self.current_id;
        self.current_id += 1;

        self.codec.write_async(request_id, command).await?;
        self.codec
            .flush_async()
            .await
            .map_err(|err| WriteError::Codec(StreamError::Io(err)))?;

        Ok(request_id)
    }
}

impl<S: Read> BsonCommandSession<S> {
    /// Read next [BsonCommand]
    pub fn read(&mut self) -> Result<ReadBsonCommand<Document>, ReadError> {
        if let Some(next_id) = self.read_map.keys().next().copied() {
            Ok(ReadBsonCommand {
                id: next_id,
                command: self.read_map.shift_remove(&next_id).unwrap()
            })
        } else {
            let read = self.codec.read()?;
            Ok(read)
        }
    }

    /// Read [BsonCommand] response
    pub fn response(&mut self, id: i32) -> Result<BsonCommand<Document>, ReadError> {
        if let Some(read) = self.read_map.shift_remove(&id) {
            return Ok(read);
        }

        loop {
            let ReadBsonCommand { id: request_id, command } = self.codec.read()?;

            if request_id == id {
                return Ok(command);
            } else {
                self.read_map.insert(request_id, command);
            }
        }
    }
}

impl<S: AsyncRead + Unpin> BsonCommandSession<S> {
    /// Read next [BsonCommand] asynchronously
    pub async fn read_async(&mut self) -> Result<ReadBsonCommand<Document>, ReadError> {
        if let Some(next_id) = self.read_map.keys().next().copied() {
            Ok(ReadBsonCommand {
                id: next_id,
                command: self.read_map.shift_remove(&next_id).unwrap()
            })
        } else {
            let read = self.codec.read_async().await?;
            Ok(read)
        }
    }

    /// Read [BsonCommand] response asynchronously
    pub async fn response_async(&mut self, id: i32) -> Result<BsonCommand<Document>, ReadError> {
        if let Some(read) = self.read_map.shift_remove(&id) {
            return Ok(read);
        }

        loop {
            let ReadBsonCommand { id: request_id, command } = self.codec.read_async().await?;

            if request_id == id {
                return Ok(command);
            } else {
                self.read_map.insert(request_id, command);
            }
        }
    }
}
