/*
 * Created on Thu Jul 29 2021
 *
 * Copyright (c) storycraft. Licensed under the MIT Licence.
 */

use std::io::{Read, Write};

use crate::{
    command::{session::BsonCommandSession, BsonCommand},
    request, response,
};

use super::{RequestResult, LocoSessionExt};

#[derive(Debug)]
pub struct CheckinClient<'a, S>(pub &'a mut BsonCommandSession<S>);

impl<S: Write + Read> CheckinClient<'_, S> {
    pub fn checkin(
        &mut self,
        checkin: &request::checkin::Checkin,
    ) -> RequestResult<response::checkin::Checkin> {
        self.0.request_response(&BsonCommand::new_const("CHECKIN", 0, checkin))
    }
    
    pub fn buy_cs(
        &mut self,
        buy_cs: &request::checkin::BuyCS,
    ) -> RequestResult<response::checkin::Checkin> {
        self.0.request_response(&BsonCommand::new_const("BUYCS", 0, buy_cs))
    }
}
