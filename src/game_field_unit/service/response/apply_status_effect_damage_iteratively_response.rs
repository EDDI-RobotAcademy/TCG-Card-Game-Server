use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApplyStatusEffectDamageIterativelyResponse {
    is_success: bool
}

impl ApplyStatusEffectDamageIterativelyResponse {
    pub fn new(is_success: bool) -> Self {
        ApplyStatusEffectDamageIterativelyResponse { is_success }
    }

    pub fn is_success(&self) -> bool {
        self.is_success
    }
}
