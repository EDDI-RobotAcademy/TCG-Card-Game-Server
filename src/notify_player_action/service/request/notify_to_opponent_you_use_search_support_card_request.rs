#[derive(Debug)]
pub struct NotifyOpponentYouUseSearchSupportRequest {
    opponent_unique_id: i32,
    usage_hand_card_id: i32,
    found_card_count: i32,
}

impl NotifyOpponentYouUseSearchSupportRequest {
    pub fn new(opponent_unique_id: i32, usage_hand_card_id: i32, found_card_count: i32) -> Self {
        NotifyOpponentYouUseSearchSupportRequest {
            opponent_unique_id,
            usage_hand_card_id,
            found_card_count,
        }
    }
    pub fn get_opponent_unique_id(&self) -> i32 { self.opponent_unique_id }
    pub fn get_usage_hand_card_id(&self) -> i32 { self.usage_hand_card_id }
    pub fn get_found_card_count(&self) -> i32 { self.found_card_count }
}