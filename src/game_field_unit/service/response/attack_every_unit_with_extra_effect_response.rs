use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AttackEveryUnitWithExtraEffectResponse {
    is_success: bool
}

impl AttackEveryUnitWithExtraEffectResponse {
    pub fn new(is_success: bool) -> Self {
        AttackEveryUnitWithExtraEffectResponse { is_success }
    }

    pub fn is_success(&self) -> bool {
        self.is_success
    }
}