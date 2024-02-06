use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct UseSupportCardResponse {
    is_success: bool,
}

impl UseSupportCardResponse {
    pub fn new(is_success: bool) -> Self {
        UseSupportCardResponse { is_success }
    }
}
