/*
 * Created on Thu Jul 29 2021
 *
 * Copyright (c) storycraft. Licensed under the MIT Licence.
 */

use futures::{AsyncRead, AsyncWrite};

use crate::{command::session::BsonCommandSession, request, response};

use super::client_method;

#[derive(Debug)]
pub struct BookingClient<'a, S>(pub &'a mut BsonCommandSession<S>);

impl<S: AsyncRead + AsyncWrite + Unpin> BookingClient<'_, S> {
    client_method!(get_conf, "GETCONF", request::booking::GetConfReq => response::booking::GetConfRes);
}
