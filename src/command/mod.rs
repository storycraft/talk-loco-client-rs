/*
 * Created on Wed Jul 28 2021
 *
 * Copyright (c) storycraft. Licensed under the MIT Licence.
 */

pub mod codec;
pub mod session;

use std::borrow::Cow;

use bson::Document;
use serde::de::DeserializeOwned;

#[derive(Debug, Clone)]
pub struct BsonCommand<T> {
    pub method: Cow<'static, str>,
    pub data_type: i8,
    pub data: T,
}

impl<T> BsonCommand<T> {
    pub const fn new(method: String, data_type: i8, data: T) -> Self {
        Self {
            method: Cow::Owned(method),
            data_type,
            data
        }
    }
    
    pub const fn new_const(method: &'static str, data_type: i8, data: T) -> Self {
        Self {
            method: Cow::Borrowed(method),
            data_type,
            data
        }
    }
}

impl BsonCommand<Document> {
    /// Deserialize [bson::Document] data
    pub fn try_deserialize<D: DeserializeOwned>(self) -> Result<BsonCommand<D>, bson::de::Error> {
        Ok(
            BsonCommand::<D> {
                method: self.method,
                data_type: self.data_type,
                data: bson::from_document(self.data)?
            }
        )
    }
}

#[derive(Debug, Clone)]
pub struct ReadBsonCommand<T> {
    pub read_id: i32,
    pub command: BsonCommand<T>
}
