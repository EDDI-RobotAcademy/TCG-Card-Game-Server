use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccountDeleteResponse {
    is_success: bool,
}

impl AccountDeleteResponse {
    pub fn new(is_success: bool) -> Self {
        AccountDeleteResponse { is_success }
    }
}