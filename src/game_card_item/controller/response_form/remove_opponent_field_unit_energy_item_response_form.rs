use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RemoveOpponentFieldUnitEnergyItemResponseForm {
    is_success: bool,
}

impl RemoveOpponentFieldUnitEnergyItemResponseForm {
    pub fn new(is_success: bool) -> Self { RemoveOpponentFieldUnitEnergyItemResponseForm { is_success } }
    pub fn get_is_success(&self) -> bool { self.is_success }
}