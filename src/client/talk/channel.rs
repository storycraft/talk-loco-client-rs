/*
 * Created on Wed Dec 08 2021
 *
 * Copyright (c) storycraft. Licensed under the MIT Licence.
 */

use std::io::{Read, Write};

use futures::{AsyncRead, AsyncWrite};

use crate::command::session::BsonCommandSession;

#[derive(Debug)]
pub struct TalkChannelClient<'a, S>(pub i64, pub &'a mut BsonCommandSession<S>);

impl<S: Write + Read> TalkChannelClient<'_, S> {}

impl<S: AsyncWrite + AsyncRead + Unpin> TalkChannelClient<'_, S> {}
