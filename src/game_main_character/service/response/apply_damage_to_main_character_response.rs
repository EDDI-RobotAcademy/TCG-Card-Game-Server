use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct ApplyDamageToMainCharacterResponse {
    is_success: bool,
}

impl ApplyDamageToMainCharacterResponse {
    pub fn new(is_success: bool) -> Self {
        ApplyDamageToMainCharacterResponse { is_success }
    }

    pub fn is_success(&self) -> bool {
        self.is_success
    }
}