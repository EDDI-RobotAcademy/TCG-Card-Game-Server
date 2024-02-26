use crate::game_field_unit::entity::attached_energy_map::AttachedEnergyMap;

#[derive(Debug)]
pub struct NoticeAttachEnergyToSpecificUnitRequest {
    opponent_unique_id: i32,
    unit_index: i32,
    updated_unit_energy_map: AttachedEnergyMap,
}

impl NoticeAttachEnergyToSpecificUnitRequest {
    pub fn new(opponent_unique_id: i32,
               unit_index: i32,
               updated_unit_energy_map: AttachedEnergyMap) -> Self {
        NoticeAttachEnergyToSpecificUnitRequest {
            opponent_unique_id,
            unit_index,
            updated_unit_energy_map
        }
    }

    pub fn get_opponent_unique_id(&self) -> i32 { self.opponent_unique_id }

    pub fn get_unit_index(&self) -> i32 { self.unit_index }

    pub fn get_updated_unit_energy_map(&self) -> &AttachedEnergyMap { &self.updated_unit_energy_map }
}