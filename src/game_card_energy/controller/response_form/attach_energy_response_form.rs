use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AttachEnergyResponseForm {
    is_success: bool,
}

impl AttachEnergyResponseForm {
    pub fn new(is_success: bool) -> Self { AttachEnergyResponseForm { is_success } }
    pub fn get_is_success(&self) -> bool { self.is_success }
}
