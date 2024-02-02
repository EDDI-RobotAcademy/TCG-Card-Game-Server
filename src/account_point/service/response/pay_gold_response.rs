use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PayGoldResponse {
    is_success: bool,
}

impl PayGoldResponse {
    pub fn new(is_success: bool) -> Self {
        PayGoldResponse { is_success }
    }
}