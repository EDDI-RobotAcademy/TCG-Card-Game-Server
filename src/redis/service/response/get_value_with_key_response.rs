use serde::{Deserialize, Serialize};
use crate::redis::service::request::save_key_and_value_request::SaveKeyAndValueRequest;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetValueWithKeyResponse {
    value: String,
}

impl GetValueWithKeyResponse {
    pub fn new(value: String) -> Self { GetValueWithKeyResponse { value } }
}