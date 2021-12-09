/*
 * Created on Thu Jul 29 2021
 *
 * Copyright (c) storycraft. Licensed under the MIT Licence.
 */

use std::io::{Read, Write};

use futures::{AsyncRead, AsyncWrite};

use crate::{
    command::{session::BsonCommandSession, BsonCommand},
    request, response,
};

use super::RequestResult;

#[derive(Debug)]
pub struct BookingClient<'a, S>(pub &'a mut BsonCommandSession<S>);

impl<S: Read + Write> BookingClient<'_, S> {
    pub fn get_conf(
        &mut self,
        get_conf: &request::booking::GetConf,
    ) -> RequestResult<response::booking::GetConf> {
        let req = self
            .0
            .request(&BsonCommand::new_const("GETCONF", 0, get_conf))?;

        Ok(self.0.response(req)?.try_deserialize()?)
    }
}

impl<S: AsyncRead + AsyncWrite + Unpin> BookingClient<'_, S> {
    pub async fn get_conf_async(
        &mut self,
        get_conf: &request::booking::GetConf,
    ) -> RequestResult<response::booking::GetConf> {
        let req = self
            .0
            .request_async(&BsonCommand::new_const("GETCONF", 0, get_conf)).await?;

        Ok(self.0.response_async(req).await?.try_deserialize()?)
    }
}
