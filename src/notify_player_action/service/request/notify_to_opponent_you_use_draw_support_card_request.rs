#[derive(Debug)]
pub struct NotifyToOpponentYouUseDrawSupportCardRequest {
    opponent_unique_id: i32,
    usage_hand_card_id: i32,
    draw_card_count: i32,
}

impl NotifyToOpponentYouUseDrawSupportCardRequest {
    pub fn new(opponent_unique_id: i32, usage_hand_card_id: i32, draw_card_count: i32) -> Self {
        NotifyToOpponentYouUseDrawSupportCardRequest {
            opponent_unique_id,
            usage_hand_card_id,
            draw_card_count,
        }
    }
    pub fn get_opponent_unique_id(&self) -> i32 { self.opponent_unique_id }
    pub fn get_usage_hand_card_id(&self) -> i32 { self.usage_hand_card_id }
    pub fn get_draw_card_count(&self) -> i32 { self.draw_card_count }
}