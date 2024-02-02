use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccountDeckConfigurationResponseForm {
    is_success: bool,
    message: String
}

impl AccountDeckConfigurationResponseForm {
    pub fn new(is_success: bool, message: String) -> Self {
        AccountDeckConfigurationResponseForm { is_success, message }
    }
    pub fn get_is_success(&self) -> bool { self.is_success }
    pub fn get_message(&self) -> &str { &self.message }
}