use crate::game_field_unit::entity::attached_energy_map::AttachedEnergyMap;

#[derive(Debug)]
pub struct NoticeRemoveEnergyOfSpecificOpponentUnitRequest {
    opponent_unique_id: i32,
    opponent_unit_index: i32,
    updated_opponent_unit_energy_map: AttachedEnergyMap,
}

impl NoticeRemoveEnergyOfSpecificOpponentUnitRequest {
    pub fn new(opponent_unique_id: i32,
               opponent_unit_index: i32,
               updated_opponent_unit_energy_map: AttachedEnergyMap) -> Self {
        NoticeRemoveEnergyOfSpecificOpponentUnitRequest {
            opponent_unique_id,
            opponent_unit_index,
            updated_opponent_unit_energy_map
        }
    }

    pub fn get_opponent_unique_id(&self) -> i32 { self.opponent_unique_id }

    pub fn get_opponent_unit_index(&self) -> i32 { self.opponent_unit_index }

    pub fn get_updated_opponent_unit_energy_map(&self) -> &AttachedEnergyMap {
        &self.updated_opponent_unit_energy_map
    }
}