#[derive(Debug)]
pub struct NotifyToOpponentYouUseItemCardRequest {
    opponent_unique_id: i32,
    opponent_target_unit_index: i32,
    usage_item_card_id: i32,
}

impl NotifyToOpponentYouUseItemCardRequest {
    pub fn new(opponent_unique_id: i32, opponent_target_unit_index: i32, usage_item_card_id: i32) -> Self {
        NotifyToOpponentYouUseItemCardRequest {
            opponent_unique_id,
            opponent_target_unit_index,
            usage_item_card_id
        }
    }

    pub fn get_opponent_unique_id(&self) -> i32 {
        self.opponent_unique_id
    }

    pub fn get_opponent_target_unit_index(&self) -> i32 {
        self.opponent_target_unit_index
    }

    pub fn get_usage_item_card_id(&self) -> i32 {
        self.usage_item_card_id
    }
}
