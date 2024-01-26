use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccountModifyResponse {
    is_success: bool,
}

impl AccountModifyResponse {
    pub fn new(is_success: bool) -> Self {
        AccountModifyResponse { is_success }
    }
}