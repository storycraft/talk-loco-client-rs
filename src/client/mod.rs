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

use futures::{AsyncRead, AsyncWrite};
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

/// Convenience method for requesting command
#[inline]
pub fn request_response<D: DeserializeOwned>(
    session: &mut BsonCommandSession<impl Read + Write>,
    command: &BsonCommand<impl Serialize>,
) -> RequestResult<D> {
    let req = session.request(command)?;
    Ok(session.response(req)?.try_deserialize()?)
}

/// Convenience method for requesting command asynchronously
pub async fn request_response_async<D: DeserializeOwned>(
    session: &mut BsonCommandSession<impl AsyncRead + AsyncWrite + Unpin>,
    command: &BsonCommand<impl Serialize>,
) -> RequestResult<D> {
    let req = session.request_async(command).await?;
    Ok(session.response_async(req).await?.try_deserialize()?)
}

macro_rules! client_method {
    (
        $(#[$meta:meta])*
        $name: ident, $method: literal, $request: ty => $response: ty
    ) => {
        $(#[$meta])*
        pub async fn $name(
            &mut self,
            command: &$request,
        ) -> crate::client::RequestResult<$response> {
            crate::client::request_response_async(
                self.0,
                &crate::command::BsonCommand::new_const($method, 0, command),
            )
            .await
        }
    };

    ($name: ident, $method: literal, $request: ty) => {
        client_method!($name, $method, $request => ());
    };
}

use client_method;
