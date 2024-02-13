use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddFieldEnergyWithFieldUnitHealthPointResponseForm {
    is_success: bool,
}

impl AddFieldEnergyWithFieldUnitHealthPointResponseForm {
    pub fn new(is_success: bool) -> Self { AddFieldEnergyWithFieldUnitHealthPointResponseForm { is_success } }
    pub fn get_is_success(&self) -> bool { self.is_success }
}