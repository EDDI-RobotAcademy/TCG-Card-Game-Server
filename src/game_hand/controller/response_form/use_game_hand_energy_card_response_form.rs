use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UseGameHandEnergyCardResponseForm {
    is_success: bool,
}

impl UseGameHandEnergyCardResponseForm {
    pub fn new(is_success: bool) -> Self { UseGameHandEnergyCardResponseForm { is_success } }
    pub fn get_is_success(&self) -> bool { self.is_success }
}