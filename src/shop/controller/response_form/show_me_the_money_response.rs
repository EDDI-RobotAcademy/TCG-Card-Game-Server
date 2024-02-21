use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShowMeTheMoneyResponse {
    is_success: bool
}

impl ShowMeTheMoneyResponse {
    pub fn new(is_success: bool) -> Self {
        ShowMeTheMoneyResponse { is_success }
    }
    pub fn get_is_success(&self) -> bool { self.is_success }
}