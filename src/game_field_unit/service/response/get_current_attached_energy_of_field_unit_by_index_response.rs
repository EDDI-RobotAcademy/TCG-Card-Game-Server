use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetCurrentAttachedEnergyOfFieldUnitByIndexResponse {
    current_attached_energy: i32,
}

impl GetCurrentAttachedEnergyOfFieldUnitByIndexResponse {
    pub fn new(current_attached_energy: i32) -> Self {
        GetCurrentAttachedEnergyOfFieldUnitByIndexResponse {
            current_attached_energy
        }
    }
    pub fn get_current_attached_energy(&self) -> i32 { self.current_attached_energy }
}