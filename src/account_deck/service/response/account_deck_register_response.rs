use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccountDeckRegisterResponse {
    is_success: bool,
}

impl AccountDeckRegisterResponse {
    pub fn new(is_success: bool) -> Self {
        AccountDeckRegisterResponse { is_success }
    }

    pub fn get_is_success(&self) -> bool { self.is_success }
}