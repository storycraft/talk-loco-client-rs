/*
 * Created on Thu Jul 29 2021
 *
 * Copyright (c) storycraft. Licensed under the MIT Licence.
 */

pub mod booking;
pub mod checkin;
pub mod talk;

pub mod media;

use std::{
    error::Error,
    fmt::Display,
    io::{Read, Write},
};

use serde::{de::DeserializeOwned, Serialize};

use crate::{
    command::{
        manager::{ReadError, WriteError},
        session::BsonCommandSession,
        BsonCommand,
    },
    response::ResponseData,
};

#[derive(Debug)]
pub enum RequestError {
    Write(WriteError),
    Read(ReadError),
    Deserialize(bson::de::Error),
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

impl From<bson::de::Error> for RequestError {
    fn from(err: bson::de::Error) -> Self {
        Self::Deserialize(err)
    }
}

impl Display for RequestError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RequestError::Write(err) => err.fmt(f),
            RequestError::Read(err) => err.fmt(f),
            RequestError::Deserialize(err) => err.fmt(f),
        }
    }
}

impl Error for RequestError {}

pub type RequestResult<T> = Result<BsonCommand<ResponseData<T>>, RequestError>;

pub trait LocoSessionExt {
    fn request_response<D: DeserializeOwned>(
        &mut self,
        command: &BsonCommand<impl Serialize>,
    ) -> RequestResult<D>;
}

impl<S: Write + Read> LocoSessionExt for BsonCommandSession<S> {
    fn request_response<D: DeserializeOwned>(
        &mut self,
        command: &BsonCommand<impl Serialize>,
    ) -> RequestResult<D> {
        let req = self.request(&command)?;

        Ok(self.response(req)?.try_deserialize()?)
    }
}
