use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CheckedResponseResponse {
    is_success: bool,
}

impl CheckedResponseResponse {
    pub fn new(is_success: bool) -> Self {
        CheckedResponseResponse { is_success }
    }

    pub fn get_is_success(&self) -> bool { self.is_success }
}