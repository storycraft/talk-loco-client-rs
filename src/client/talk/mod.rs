/*
 * Created on Thu Jul 29 2021
 *
 * Copyright (c) storycraft. Licensed under the MIT Licence.
 */

mod channel;

use std::io::{Read, Write};

use futures::{AsyncRead, AsyncWrite};

use crate::command::session::BsonCommandSession;

#[derive(Debug)]
pub struct TalkClient<'a, S>(pub &'a mut BsonCommandSession<S>);

impl<S: Write + Read> TalkClient<'_, S> {}

impl<S: AsyncWrite + AsyncRead + Unpin> TalkClient<'_, S> {}
