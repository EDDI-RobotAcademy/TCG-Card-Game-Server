use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccountDeckDeleteResponse {
    is_success: bool
}

impl AccountDeckDeleteResponse {
    pub fn new(is_success: bool) -> Self { AccountDeckDeleteResponse { is_success } }
    pub fn get_is_success(&self) -> bool { self.is_success }
}