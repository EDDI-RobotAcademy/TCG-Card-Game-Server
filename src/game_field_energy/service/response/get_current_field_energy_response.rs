use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetCurrentFieldEnergyResponse {
    field_energy_count: i32,
}

impl GetCurrentFieldEnergyResponse {
    pub fn new(field_energy_count: i32) -> Self { GetCurrentFieldEnergyResponse { field_energy_count } }
    pub fn get_field_energy_count(&self) -> i32 { self.field_energy_count }
}