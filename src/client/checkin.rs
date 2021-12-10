/*
 * Created on Thu Jul 29 2021
 *
 * Copyright (c) storycraft. Licensed under the MIT Licence.
 */

use futures::{AsyncRead, AsyncWrite};

use crate::{command::session::BsonCommandSession, request, response};

use super::client_method;

#[derive(Debug)]
pub struct CheckinClient<'a, S>(pub &'a mut BsonCommandSession<S>);

impl<S: AsyncWrite + AsyncRead + Unpin> CheckinClient<'_, S> {
    client_method!(checkin, "CHECKIN", request::checkin::Checkin => response::checkin::Checkin);

    client_method!(buy_cs, "BUYCS", request::checkin::BuyCS => response::checkin::BuyCS);
}
