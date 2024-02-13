use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddFieldEnergyWithAmountResponse {
    is_success: bool,
}

impl AddFieldEnergyWithAmountResponse {
    pub fn new(is_success: bool) -> Self { AddFieldEnergyWithAmountResponse { is_success } }
    pub fn is_success(&self) -> bool { self.is_success }
}