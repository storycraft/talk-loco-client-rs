/*
 * Created on Thu Jul 29 2021
 *
 * Copyright (c) storycraft. Licensed under the MIT Licence.
 */

use futures::{AsyncRead, AsyncWrite};

use crate::{
    command::{session::BsonCommandSession, BsonCommand},
    request, response,
};

use super::{RequestResult, request_response_async};

#[derive(Debug)]
pub struct BookingClient<'a, S>(pub &'a mut BsonCommandSession<S>);

impl<S: AsyncRead + AsyncWrite + Unpin> BookingClient<'_, S> {
    pub async fn get_conf(
        &mut self,
        get_conf: &request::booking::GetConf,
    ) -> RequestResult<response::booking::GetConf> {
        request_response_async(&mut self.0, &BsonCommand::new_const("GETCONF", 0, get_conf)).await
    }
}
