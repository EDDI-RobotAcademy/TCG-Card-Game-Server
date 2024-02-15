#[derive(Debug)]
pub struct NotifyToOpponentYouUseDestroyDeckItemCardRequest {
    opponent_unique_id: i32,
    usage_hand_card_id: i32,
    will_be_lost_card: i32,
}

impl NotifyToOpponentYouUseDestroyDeckItemCardRequest {
    pub fn new(opponent_unique_id: i32, usage_hand_card_id: i32, will_be_lost_card: i32) -> Self {
        NotifyToOpponentYouUseDestroyDeckItemCardRequest {
            opponent_unique_id,
            usage_hand_card_id,
            will_be_lost_card,
        }
    }
    pub fn get_opponent_unique_id(&self) -> i32 { self.opponent_unique_id }
    pub fn get_usage_hand_card_id(&self) -> i32 { self.usage_hand_card_id }
    pub fn get_will_be_lost_card(&self) -> i32 { self.will_be_lost_card }
}