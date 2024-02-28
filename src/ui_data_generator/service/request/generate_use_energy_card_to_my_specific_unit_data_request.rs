use crate::game_field_unit::entity::attached_energy_map::AttachedEnergyMap;

#[derive(Debug)]
pub struct GenerateUseEnergyCardToMySpecificUnitDataRequest {
    used_hand_card_id: i32,
    unit_index: i32,
    updated_unit_energy_map: AttachedEnergyMap,
}

impl GenerateUseEnergyCardToMySpecificUnitDataRequest {
    pub fn new(used_hand_card_id: i32,
               unit_index: i32,
               updated_unit_energy_map: AttachedEnergyMap,) -> Self {
        GenerateUseEnergyCardToMySpecificUnitDataRequest {
            used_hand_card_id,
            unit_index,
            updated_unit_energy_map,
        }
    }

    pub fn get_used_hand_card_id(&self) -> i32 { self.used_hand_card_id }

    pub fn get_unit_index(&self) -> i32 { self.unit_index }

    pub fn get_updated_unit_energy_map(&self) -> &AttachedEnergyMap {
        &self.updated_unit_energy_map
    }
}