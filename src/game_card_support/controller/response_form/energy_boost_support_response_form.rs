use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnergyBoostSupportResponseForm {
    is_success: bool,
}

impl EnergyBoostSupportResponseForm {
    pub fn new(is_success: bool) -> Self { EnergyBoostSupportResponseForm { is_success } }
    pub fn get_is_success(&self) -> bool { self.is_success }
}