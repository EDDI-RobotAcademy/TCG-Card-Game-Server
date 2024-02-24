#[derive(Debug)]
pub struct NoticeAttachEnergyToSpecificUnitByUsingHandCardRequest {
    account_unique_id: i32,
    opponent_unique_id: i32,
    used_hand_card_id: i32,
    unit_index: i32,
}

impl NoticeAttachEnergyToSpecificUnitByUsingHandCardRequest {
    pub fn new(account_unique_id: i32,
               opponent_unique_id: i32,
               used_hand_card_id: i32,
               unit_index: i32) -> Self {
        NoticeAttachEnergyToSpecificUnitByUsingHandCardRequest {
            account_unique_id,
            opponent_unique_id,
            used_hand_card_id,
            unit_index,
        }
    }

    pub fn get_account_unique_id(&self) -> i32 { self.account_unique_id }

    pub fn get_opponent_unique_id(&self) -> i32 { self.opponent_unique_id }

    pub fn get_used_hand_card_id(&self) -> i32 { self.used_hand_card_id }

    pub fn get_unit_index(&self) -> i32 { self.unit_index }
}