use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AttachSpecialEnergyCardResponseForm {
    is_success: bool,
}

impl AttachSpecialEnergyCardResponseForm {
    pub fn new(is_success: bool) -> Self { AttachSpecialEnergyCardResponseForm { is_success } }
    pub fn is_success(&self) -> bool { self.is_success }
}
