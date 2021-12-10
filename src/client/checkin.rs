/*
 * Created on Thu Jul 29 2021
 *
 * Copyright (c) storycraft. Licensed under the MIT Licence.
 */

use futures::{AsyncWrite, AsyncRead};

use crate::{
    command::{session::BsonCommandSession, BsonCommand},
    request, response,
};

use super::{RequestResult, request_response_async};

#[derive(Debug)]
pub struct CheckinClient<'a, S>(pub &'a mut BsonCommandSession<S>);

impl<S: AsyncWrite + AsyncRead + Unpin> CheckinClient<'_, S> {
    pub async fn checkin(
        &mut self,
        checkin: &request::checkin::Checkin,
    ) -> RequestResult<response::checkin::Checkin> {
        request_response_async(&mut self.0, &BsonCommand::new_const("CHECKIN", 0, checkin)).await
    }

    pub async fn buy_cs(
        &mut self,
        buy_cs: &request::checkin::BuyCS,
    ) -> RequestResult<response::checkin::Checkin> {
        request_response_async(&mut self.0, &BsonCommand::new_const("BUYCS", 0, buy_cs)).await
    }
}
