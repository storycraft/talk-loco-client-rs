/*
 * Created on Thu Dec 03 2020
 *
 * Copyright (c) storycraft. Licensed under the MIT Licence.
 */

use serde::{Serialize, Deserialize};
use crate::{ structs::chat::Chatlog};

/// Send when new user join.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewMem {
    
    /// Join feed chat.(?)
    #[serde(rename = "chatLog")]
    pub chat_log: Chatlog

}