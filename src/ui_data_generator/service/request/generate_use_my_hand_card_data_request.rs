#[derive(Debug)]
pub struct GenerateUseMyHandCardDataRequest {
    used_hand_card_id: i32,
}

impl GenerateUseMyHandCardDataRequest {
    pub fn new(used_hand_card_id: i32) -> Self {
        GenerateUseMyHandCardDataRequest {
            used_hand_card_id,
        }
    }

    pub fn get_used_hand_card_id(&self) -> i32 { self.used_hand_card_id }
}