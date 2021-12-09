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
pub struct CheckinClient<'a, S>(pub &'a mut BsonCommandSession<S>);

impl<S: Write + Read> CheckinClient<'_, S> {
    pub fn checkin(
        &mut self,
        checkin: &request::checkin::Checkin,
    ) -> RequestResult<response::checkin::Checkin> {
        let req = self
            .0
            .request(&BsonCommand::new_const("CHECKIN", 0, checkin))?;

        Ok(self.0.response(req)?.try_deserialize()?)
    }
    
    pub fn buy_cs(
        &mut self,
        buy_cs: &request::checkin::BuyCS,
    ) -> RequestResult<response::checkin::Checkin> {
        let req = self
            .0
            .request(&BsonCommand::new_const("BUYCS", 0, buy_cs))?;

        Ok(self.0.response(req)?.try_deserialize()?)
    }
}

impl<S: AsyncWrite + AsyncRead + Unpin> CheckinClient<'_, S> {
    pub async fn checkin_async(
        &mut self,
        checkin: &request::checkin::Checkin,
    ) -> RequestResult<response::checkin::Checkin> {
        let req = self
            .0
            .request_async(&BsonCommand::new_const("CHECKIN", 0, checkin))
            .await?;

        Ok(self.0.response_async(req).await?.try_deserialize()?)
    }

    pub async fn buy_cs_async(
        &mut self,
        buy_cs: &request::checkin::BuyCS,
    ) -> RequestResult<response::checkin::Checkin> {
        let req = self
            .0
            .request_async(&BsonCommand::new_const("BUYCS", 0, buy_cs)).await?;

        Ok(self.0.response_async(req).await?.try_deserialize()?)
    }
}
