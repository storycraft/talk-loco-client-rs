/*
 * Created on Thu Dec 03 2020
 *
 * Copyright (c) storycraft. Licensed under the MIT Licence.
 */

use serde::{Serialize, Deserialize};
use crate::{ structs::channel_info::ChannelMeta};

/// Sync Chatroom meta update
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChgMeta {

    /// Chatroom id
    #[serde(rename = "chatId")]
    pub chat_id: i64,

    /// Chatroom meta item. Update same type meta.
    pub meta: ChannelMeta

}