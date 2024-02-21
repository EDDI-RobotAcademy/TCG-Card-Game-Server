use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateAccountCardDbResponse {
    is_success: bool,
}

impl UpdateAccountCardDbResponse {
    pub fn new(is_success: bool) -> Self {
        UpdateAccountCardDbResponse { is_success }
    }
    pub fn get_is_success(&self) -> bool { self.is_success }
}