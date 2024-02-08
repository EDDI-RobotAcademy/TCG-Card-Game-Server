use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AttachGeneralEnergyCardResponseForm {
    is_success: bool,
}

impl AttachGeneralEnergyCardResponseForm {
    pub fn new(is_success: bool) -> Self { AttachGeneralEnergyCardResponseForm { is_success } }
    pub fn get_is_success(&self) -> bool { self.is_success }
}
