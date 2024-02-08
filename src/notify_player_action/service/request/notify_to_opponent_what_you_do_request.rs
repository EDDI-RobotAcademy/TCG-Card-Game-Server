use crate::game_hand::service::request::use_game_hand_unit_card_request::UseGameHandUnitCardRequest;

#[derive(Debug)]
pub struct NotifyToOpponentWhatYouDoRequest {
    opponent_unique_id: i32,
    usage_hand_card_id: i32,
}

impl NotifyToOpponentWhatYouDoRequest {
    pub fn new(opponent_unique_id: i32, usage_hand_card_id: i32) -> Self {
        NotifyToOpponentWhatYouDoRequest {
            opponent_unique_id,
            usage_hand_card_id,
        }
    }

    pub fn get_opponent_unique_id(&self) -> i32 {
        self.opponent_unique_id
    }

    pub fn get_usage_hand_card_id(&self) -> i32 {
        self.usage_hand_card_id
    }
}
