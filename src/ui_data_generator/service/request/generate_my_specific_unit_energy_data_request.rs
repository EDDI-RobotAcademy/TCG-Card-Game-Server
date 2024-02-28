use crate::game_field_unit::entity::attached_energy_map::AttachedEnergyMap;

#[derive(Debug)]
pub struct GenerateMySpecificUnitEnergyDataRequest {
    unit_index: i32,
    updated_unit_energy_map: AttachedEnergyMap,
}

impl GenerateMySpecificUnitEnergyDataRequest {
    pub fn new(unit_index: i32,
               updated_unit_energy_map: AttachedEnergyMap,) -> Self {
        GenerateMySpecificUnitEnergyDataRequest {
            unit_index,
            updated_unit_energy_map,
        }
    }

    pub fn get_unit_index(&self) -> i32 { self.unit_index }

    pub fn get_updated_unit_energy_map(&self) -> &AttachedEnergyMap {
        &self.updated_unit_energy_map
    }
}