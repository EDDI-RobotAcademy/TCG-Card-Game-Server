use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AttackUnitResponseForm {
    is_success: bool,
}

impl AttackUnitResponseForm {
    pub fn new(is_success: bool) -> Self { AttackUnitResponseForm { is_success } }
    pub fn get_is_success(&self) -> bool { self.is_success }
}
