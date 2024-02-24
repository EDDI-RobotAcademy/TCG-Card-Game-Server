use serde::{Deserialize, Serialize};
use crate::game_field_unit::entity::attached_energy_map::AttachedEnergyMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetCurrentAttachedEnergyOfFieldUnitByIndexResponse {
    current_attached_energy_map: AttachedEnergyMap,
}

impl GetCurrentAttachedEnergyOfFieldUnitByIndexResponse {
    pub fn new(current_attached_energy_map: AttachedEnergyMap) -> Self {
        GetCurrentAttachedEnergyOfFieldUnitByIndexResponse {
            current_attached_energy_map
        }
    }
    pub fn get_current_attached_energy_map(&self) -> &AttachedEnergyMap { &self.current_attached_energy_map }
}