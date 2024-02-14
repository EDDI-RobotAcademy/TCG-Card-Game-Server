use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AttackTargetUnitWithExtraEffectResponse {
    is_success: bool
}

impl AttackTargetUnitWithExtraEffectResponse {
    pub fn new(is_success: bool) -> Self {
        AttackTargetUnitWithExtraEffectResponse { is_success }
    }

    pub fn is_success(&self) -> bool {
        self.is_success
    }
}
