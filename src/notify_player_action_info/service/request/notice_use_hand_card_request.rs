#[derive(Debug)]
pub struct NoticeUseHandCardRequest {
    opponent_unique_id: i32,
    used_hand_card_id: i32,
}

impl NoticeUseHandCardRequest {
    pub fn new(opponent_unique_id: i32,
               used_hand_card_id: i32) -> Self {
        NoticeUseHandCardRequest {
            opponent_unique_id,
            used_hand_card_id,
        }
    }

    pub fn get_opponent_unique_id(&self) -> i32 { self.opponent_unique_id }

    pub fn get_used_hand_card_id(&self) -> i32 { self.used_hand_card_id }
}