/*
 * Created on Sun Dec 12 2021
 *
 * Copyright (c) storycraft. Licensed under the MIT Licence.
 */

use serde::{Serialize, Deserialize};

use crate::request;

/// Forward chat
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ForwardReq {
    /// [request::chat::Write] content to forward
    #[serde(flatten)]
    pub content: request::chat::WriteReq
}
