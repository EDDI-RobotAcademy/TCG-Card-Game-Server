use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccountDeckConfigurationResponse {
    is_success: bool,
    message: String
}

impl AccountDeckConfigurationResponse {
    pub fn new(is_success: bool, message: String) -> Self { AccountDeckConfigurationResponse { is_success, message } }
    pub fn get_is_success(&self) -> bool { self.is_success }
    pub fn get_message(&self) -> &str { &self.message }
}