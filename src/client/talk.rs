/*
 * Created on Thu Jul 29 2021
 *
 * Copyright (c) storycraft. Licensed under the MIT Licence.
 */

use futures::{AsyncRead, AsyncWrite};

use crate::{command::session::BsonCommandSession, request, response};

use super::client_method;

#[derive(Debug)]
pub struct TalkClient<'a, S>(pub &'a mut BsonCommandSession<S>);

impl<S: AsyncWrite + AsyncRead + Unpin> TalkClient<'_, S> {
    client_method!(login, "LOGINLIST", request::chat::LoginList => response::chat::LoginList);

    client_method!(load_channel_list, "LCHATLIST", request::chat::LChatList => response::chat::LChatList);

    client_method!(set_status, "SETST", request::chat::SetSt);



    client_method!(chat_on_channel, "CHATONROOM", request::chat::ChatOnRoom => response::chat::ChatOnRoom);

    client_method!(write, "WRITE", request::chat::Write => response::chat::Write);

    client_method!(delete_chat, "DELETEMSG", request::chat::DeleteMsg => response::chat::DeleteMsg);

    client_method!(leave, "LEAVE", request::chat::Leave => response::chat::Leave);

    client_method!(read_chat, "NOTIREAD", request::chat::NotiRead);

    client_method!(set_meta, "SETMETA", request::chat::SetMeta => response::chat::SetMeta);
    
    client_method!(sync_chat, "SYNCMSG", request::chat::SyncMsg => response::chat::SyncMsg);
    
    client_method!(members, "GETMEM", request::chat::GetMem => response::chat::GetMem);

    client_method!(member_info, "MEMBER", request::chat::Member => response::chat::Member);
}
