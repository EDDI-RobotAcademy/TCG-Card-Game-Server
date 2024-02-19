use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AttachFieldEnergyToFieldUnitResponseForm {
    is_success: bool,
}

impl AttachFieldEnergyToFieldUnitResponseForm {
    pub fn new(is_success: bool) -> Self { AttachFieldEnergyToFieldUnitResponseForm { is_success } }
    pub fn get_is_success(&self) -> bool { self.is_success }
}