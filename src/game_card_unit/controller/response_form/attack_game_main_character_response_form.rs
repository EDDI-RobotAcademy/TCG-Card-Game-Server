use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AttackGameMainCharacterResponseForm {
    is_success: bool,
}

impl AttackGameMainCharacterResponseForm {
    pub fn new(is_success: bool) -> Self { AttackGameMainCharacterResponseForm { is_success } }
    pub fn get_is_success(&self) -> bool { self.is_success }
}
