/*
 * Created on Mon Dec 13 2021
 *
 * Copyright (c) storycraft. Licensed under the MIT Licence.
 */

use serde::{Serialize, Deserialize};

/// Request media download server
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetTrailerReq {
    /// Media key
    #[serde(rename = "k")]
    pub key: String,
    
    /// Chat type
    #[serde(rename = "t")]
    pub chat_type: i32
}

