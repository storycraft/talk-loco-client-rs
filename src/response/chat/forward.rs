/*
 * Created on Sun Dec 12 2021
 *
 * Copyright (c) storycraft. Licensed under the MIT Licence.
 */

use serde::{Serialize, Deserialize};

use crate::structs::chat::Chatlog;

/// [crate::request::chat::ForwardReq] response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ForwardRes {
    /// Fowarded message
    #[serde(rename = "chatLog")]
    pub chatlog: Chatlog,
}
