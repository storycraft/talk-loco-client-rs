/*
 * Created on Thu Jul 29 2021
 *
 * Copyright (c) storycraft. Licensed under the MIT Licence.
 */

pub mod booking;
pub mod checkin;
pub mod talk;

pub mod media;

use std::{error::Error, fmt::Display};

use crate::{command::{manager::{WriteError, ReadError}, BsonCommand}, response::ResponseData};

#[derive(Debug)]
pub enum RequestError {
    Write(WriteError),
    Read(ReadError),
    Deserialize(bson::de::Error)
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
            RequestError::Deserialize(err) => err.fmt(f)
        }
    }
}

impl Error for RequestError {}

pub type RequestResult<T> = Result<BsonCommand<ResponseData<T>>, RequestError>;
