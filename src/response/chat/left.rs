/*
 * Created on Thu Dec 03 2020
 *
 * Copyright (c) storycraft. Licensed under the MIT Licence.
 */

use serde::{Serialize, Deserialize};

/// Sync client left chatroom
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Left {

    /// Chatroom id
    #[serde(rename = "chatId")]
    pub chat_id: i64,

    /// Last token(?) id
    #[serde(rename = "lastTokenId")]
    pub last_token_id: i64

}