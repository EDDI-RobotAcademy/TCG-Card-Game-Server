use crate::game_field_unit::entity::attached_energy_map::AttachedEnergyMap;

#[derive(Debug)]
pub struct GenerateUseSupportCardToBoostEnergyToMySpecificUnitRequest {
    used_hand_card_id: i32,
    found_energy_card_id_list: Vec<i32>,
    unit_index: i32,
    updated_attached_energy_map: AttachedEnergyMap,
}

impl GenerateUseSupportCardToBoostEnergyToMySpecificUnitRequest {
    pub fn new(used_hand_card_id: i32,
               found_energy_card_id_list: Vec<i32>,
               unit_index: i32,
               updated_attached_energy_map: AttachedEnergyMap,) -> Self {
        GenerateUseSupportCardToBoostEnergyToMySpecificUnitRequest {
            used_hand_card_id,
            found_energy_card_id_list,
            unit_index,
            updated_attached_energy_map,
        }
    }

    pub fn get_used_hand_card_id(&self) -> i32 { self.used_hand_card_id }

    pub fn get_found_energy_card_id_list(&self) -> &Vec<i32> { &self.found_energy_card_id_list }

    pub fn get_unit_index(&self) -> i32 { self.unit_index }

    pub fn get_updated_attached_energy_map(&self) -> &AttachedEnergyMap { &self.updated_attached_energy_map }
}