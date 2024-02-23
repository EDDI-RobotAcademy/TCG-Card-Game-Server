use crate::game_field_unit::entity::attached_energy_map::AttachedEnergyMap;

#[derive(Debug)]
pub struct NoticeBoostEnergyToSpecificUnitByUsingHandCardRequest {
    account_unique_id: i32,
    opponent_unique_id: i32,
    used_hand_card_id: i32,
    found_energy_card_id_list: Vec<i32>,
    unit_index: i32,
}

impl NoticeBoostEnergyToSpecificUnitByUsingHandCardRequest {
    pub fn new(account_unique_id: i32,
               opponent_unique_id: i32,
               used_hand_card_id: i32,
               found_energy_card_id_list: Vec<i32>,
               unit_index: i32) -> Self {
        NoticeBoostEnergyToSpecificUnitByUsingHandCardRequest {
            account_unique_id,
            opponent_unique_id,
            used_hand_card_id,
            found_energy_card_id_list,
            unit_index,
        }
    }

    pub fn get_account_unique_id(&self) -> i32 { self.account_unique_id }

    pub fn get_opponent_unique_id(&self) -> i32 { self.opponent_unique_id }

    pub fn get_used_hand_card_id(&self) -> i32 { self.used_hand_card_id }

    pub fn get_found_energy_card_id_list(&self) -> &Vec<i32> { &self.found_energy_card_id_list }

    pub fn get_unit_index(&self) -> i32 { self.unit_index }
}