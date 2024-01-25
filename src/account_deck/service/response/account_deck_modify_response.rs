use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccountDeckModifyResponse {
    is_success: bool,
}

impl AccountDeckModifyResponse {
    pub fn new(is_success: bool) -> Self {
        AccountDeckModifyResponse { is_success }
    }
    pub fn get_is_success(&self) -> bool { self.is_success }
}