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
    client_method!(login, "LOGINLIST", request::chat::LoginListReq => response::chat::LoginListRes);

    client_method!(load_channel_list, "LCHATLIST", request::chat::LChatListReq => response::chat::LChatListRes);

    client_method!(set_status, "SETST", request::chat::SetStReq);

    client_method!(channel_info, "CHATINFO", request::chat::ChatInfoReq => response::chat::ChatInfoRes);

    client_method!(chat_on_channel, "CHATONROOM", request::chat::ChatOnRoomReq => response::chat::ChatOnRoomRes);

    client_method!(write, "WRITE", request::chat::WriteReq => response::chat::WriteRes);

    client_method!(forward, "FORWARD", request::chat::ForwardReq => response::chat::ForwardRes);

    client_method!(delete_chat, "DELETEMSG", request::chat::DeleteMsgReq => response::chat::DeleteMsgRes);

    client_method!(leave, "LEAVE", request::chat::LeaveReq => response::chat::LeaveRes);

    client_method!(read_chat, "NOTIREAD", request::chat::NotiReadReq);

    client_method!(set_meta, "SETMETA", request::chat::SetMetaReq => response::chat::SetMetaRes);

    client_method!(sync_chat, "SYNCMSG", request::chat::SyncMsgReq => response::chat::SyncMsgRes);

    client_method!(channel_users, "GETMEM", request::chat::GetMemReq => response::chat::GetMemRes);

    client_method!(user_info, "MEMBER", request::chat::MemberReq => response::chat::MemberRes);

    client_method!(update_channel, "UPDATECHAT", request::chat::UpdateChatReq);

    client_method!(get_trailer, "GETTRAILER", request::chat::GetTrailerReq => response::chat::GetTrailerRes);
}
