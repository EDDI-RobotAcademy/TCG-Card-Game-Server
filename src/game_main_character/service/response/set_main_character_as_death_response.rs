use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct SetMainCharacterAsDeathResponse {
    is_success: bool,
}

impl SetMainCharacterAsDeathResponse {
    pub fn new(is_success: bool) -> Self {
        SetMainCharacterAsDeathResponse { is_success }
    }

    pub fn is_success(&self) -> bool {
        self.is_success
    }
}