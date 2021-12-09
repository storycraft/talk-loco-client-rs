/*
 * Created on Sun Jul 25 2021
 *
 * Copyright (c) storycraft. Licensed under the MIT Licence.
 */

use std::{
    error::Error,
    fmt::Display,
    io::{Cursor, Read, Write},
    string::FromUtf8Error,
};

use bson::Document;
use futures::{AsyncRead, AsyncWrite};
use loco_protocol::command::{
    builder::CommandBuilder,
    codec::{CommandCodec, StreamError},
    Command,
};
use serde::Serialize;

use super::BsonCommand;

#[derive(Debug)]
pub enum WriteError {
    Codec(StreamError),
    Encode(bson::ser::Error),
}

impl From<StreamError> for WriteError {
    fn from(err: StreamError) -> Self {
        Self::Codec(err)
    }
}

impl From<bson::ser::Error> for WriteError {
    fn from(err: bson::ser::Error) -> Self {
        Self::Encode(err)
    }
}

impl Display for WriteError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            WriteError::Codec(err) => err.fmt(f),
            WriteError::Encode(err) => err.fmt(f),
        }
    }
}

impl Error for WriteError {}

#[derive(Debug)]
pub enum ReadError {
    Stream(StreamError),

    /// Response command's status is not 0, means the request is corrupted
    Corrupted(Command),

    InvalidMethod(FromUtf8Error),
    Decode(bson::de::Error),
}

impl From<StreamError> for ReadError {
    fn from(err: StreamError) -> Self {
        Self::Stream(err)
    }
}

impl From<FromUtf8Error> for ReadError {
    fn from(err: FromUtf8Error) -> Self {
        Self::InvalidMethod(err)
    }
}

impl From<bson::de::Error> for ReadError {
    fn from(err: bson::de::Error) -> Self {
        Self::Decode(err)
    }
}

impl Display for ReadError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ReadError::Stream(err) => err.fmt(f),
            ReadError::Corrupted(err) => write!(f, "Read stream corrupted. status: {}", err.header.status),
            ReadError::InvalidMethod(err) => err.fmt(f),
            ReadError::Decode(err) => err.fmt(f),
        }
    }
}

impl Error for ReadError {}

/// [BsonCommand] read / write manager
#[derive(Debug)]
pub struct BsonCommandManager<S> {
    current_id: i32,
    codec: CommandCodec<S>,
}

impl<S> BsonCommandManager<S> {
    /// Create new [BsonCommandManager] from Stream
    pub fn new(stream: S) -> Self {
        Self {
            current_id: 0,
            codec: CommandCodec::new(stream),
        }
    }

    pub fn codec(&self) -> &CommandCodec<S> {
        &self.codec
    }

    pub fn codec_mut(&mut self) -> &mut CommandCodec<S> {
        &mut self.codec
    }

    pub fn stream(&self) -> &S {
        self.codec.stream()
    }

    pub fn stream_mut(&mut self) -> &mut S {
        self.codec.stream_mut()
    }

    pub fn current_id(&self) -> i32 {
        self.current_id
    }

    pub fn into_inner(self) -> S {
        self.codec.into_inner()
    }
}

impl<S: Write> BsonCommandManager<S> {
    /// Write [BsonCommand]. returns request_id on success
    pub fn write<T: Serialize>(&mut self, command: &BsonCommand<T>) -> Result<i32, WriteError> {
        let request_id = self.current_id;
        self.current_id += 1;

        let command = encode_bson_command(request_id, command)?;

        self.codec.write(&command)?;

        Ok(request_id)
    }
}

impl<S: Read> BsonCommandManager<S> {
    /// Read [BsonCommand]. returns (request_id, [BsonCommand]) tuple
    pub fn read(&mut self) -> Result<(i32, BsonCommand<Document>), ReadError> {
        let (_, command) = self.codec.read()?;

        if command.header.status == 0 {
            let id = command.header.id;
            let method = command.header.method()?;

            let data = bson::Document::from_reader(&mut Cursor::new(command.data))?;

            Ok((id, BsonCommand::new(method, command.header.data_type, data)))
        } else {
            Err(ReadError::Corrupted(command))
        }
    }
}

impl<S: AsyncWrite + Unpin> BsonCommandManager<S> {
    /// Write [BsonCommand] async. returns request_id on success
    pub async fn write_async<T: Serialize>(
        &mut self,
        command: &BsonCommand<T>,
    ) -> Result<i32, WriteError> {
        let request_id = self.current_id;
        self.current_id += 1;

        let command = encode_bson_command(request_id, command)?;

        self.codec.write_async(&command).await?;

        Ok(request_id)
    }
}

impl<S: AsyncRead + Unpin> BsonCommandManager<S> {
    /// Read [BsonCommand]. returns (request_id, [BsonCommand]) tuple
    pub async fn read_async(&mut self) -> Result<(i32, BsonCommand<Document>), ReadError> {
        let (_, command) = self.codec.read_async().await?;

        if command.header.status == 0 {
            let id = command.header.id;
            let method = command.header.method()?;

            let data = bson::Document::from_reader(&mut Cursor::new(command.data))?;

            Ok((id, BsonCommand::new(method, command.header.data_type, data)))
        } else {
            Err(ReadError::Corrupted(command))
        }
    }
}

fn encode_bson_command<T: Serialize>(
    request_id: i32,
    command: &BsonCommand<T>,
) -> Result<Command, bson::ser::Error> {
    let builder = CommandBuilder::new(request_id, &command.method);

    let mut raw_data = Vec::new();

    let doc = bson::to_document(&command.data)?;
    doc.to_writer(&mut raw_data)?;

    Ok(builder.build(0, raw_data))
}
