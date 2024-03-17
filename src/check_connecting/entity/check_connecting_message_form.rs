use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CheckConnectingMessageForm {
    address: String,
}

impl CheckConnectingMessageForm {
    pub fn new(address: &str) -> Self {
        CheckConnectingMessageForm {
            address: address.to_string(),
        }
    }
}