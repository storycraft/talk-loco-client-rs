/*
 * Created on Thu Dec 03 2020
 *
 * Copyright (c) storycraft. Licensed under the MIT Licence.
 */

use serde::{Serialize, Deserialize};
use crate::{ structs::channel_info::ChannelListData};

/// Request every chatroom list
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LChatListRes {

    #[serde(rename = "chatDatas")]
    pub chat_datas: Vec<ChannelListData>

}