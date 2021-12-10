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
pub struct BookingClient<'a, S>(pub &'a mut BsonCommandSession<S>);

impl<S: Read + Write> BookingClient<'_, S> {
    pub fn get_conf(
        &mut self,
        get_conf: &request::booking::GetConf,
    ) -> RequestResult<response::booking::GetConf> {
        self.0.request_response(&BsonCommand::new_const("GETCONF", 0, get_conf))
    }
}
