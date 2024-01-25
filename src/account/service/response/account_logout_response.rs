use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccountLogoutResponse {
    is_success: bool,
}

impl AccountLogoutResponse {
    pub fn new(is_success: bool) -> Self {
        AccountLogoutResponse { is_success }
    }
}