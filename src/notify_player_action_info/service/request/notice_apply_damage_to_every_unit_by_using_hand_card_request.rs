#[derive(Debug)]
pub struct NoticeApplyDamageToEveryUnitByUsingHandCardRequest {
    opponent_unique_id: i32,
    used_hand_card_id: i32,
    damage: i32,
}

impl NoticeApplyDamageToEveryUnitByUsingHandCardRequest {
    pub fn new(opponent_unique_id: i32,
               used_hand_card_id: i32,
               damage: i32) -> Self {
        NoticeApplyDamageToEveryUnitByUsingHandCardRequest {
            opponent_unique_id,
            used_hand_card_id,
            damage
        }
    }

    pub fn get_opponent_unique_id(&self) -> i32 { self.opponent_unique_id }

    pub fn get_used_hand_card_id(&self) -> i32 { self.used_hand_card_id }

    pub fn get_damage(&self) -> i32 { self.damage }
}