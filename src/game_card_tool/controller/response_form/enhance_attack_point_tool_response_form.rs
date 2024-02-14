use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnhanceAttackPointToolResponseForm {
    is_success: bool,
}

impl EnhanceAttackPointToolResponseForm {
    pub fn new(is_success: bool) -> Self { EnhanceAttackPointToolResponseForm { is_success } }
    pub fn get_is_success(&self) -> bool { self.is_success }
}