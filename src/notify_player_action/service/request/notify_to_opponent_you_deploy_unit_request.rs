#[derive(Debug)]
pub struct NotifyToOpponentYouDeployUnitRequest {
    opponent_unique_id: i32,
    usage_hand_card_id: i32,
}

impl NotifyToOpponentYouDeployUnitRequest {
    pub fn new(opponent_unique_id: i32, usage_hand_card_id: i32) -> Self {
        NotifyToOpponentYouDeployUnitRequest {
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
