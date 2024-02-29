use crate::game_field_unit::entity::attached_energy_map::AttachedEnergyMap;

#[derive(Debug)]
pub struct GenerateOpponentSpecificUnitEnergyDataRequest {
    unit_index: i32,
    updated_unit_energy_map: AttachedEnergyMap,
}

impl GenerateOpponentSpecificUnitEnergyDataRequest {
    pub fn new(unit_index: i32,
               updated_unit_energy_map: AttachedEnergyMap,) -> Self {
        GenerateOpponentSpecificUnitEnergyDataRequest {
            unit_index,
            updated_unit_energy_map,
        }
    }

    pub fn get_unit_index(&self) -> i32 { self.unit_index }

    pub fn get_updated_unit_energy_map(&self) -> &AttachedEnergyMap {
        &self.updated_unit_energy_map
    }
}