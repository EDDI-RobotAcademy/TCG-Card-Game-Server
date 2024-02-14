#[derive(Debug)]
pub struct NotifyToOpponentYouUseToolCardToEnhanceAttackPointRequest {
    opponent_unique_id: i32,
    unit_card_index: i32,
    usage_tool_card_id: i32,
}

impl NotifyToOpponentYouUseToolCardToEnhanceAttackPointRequest {
    pub fn new(opponent_unique_id: i32, unit_card_index: i32, usage_tool_card_id: i32) -> Self {
        NotifyToOpponentYouUseToolCardToEnhanceAttackPointRequest {
            opponent_unique_id,
            unit_card_index,
            usage_tool_card_id
        }
    }

    pub fn get_opponent_unique_id(&self) -> i32 {
        self.opponent_unique_id
    }

    pub fn get_unit_card_index(&self) -> i32 {
        self.unit_card_index
    }

    pub fn get_usage_tool_card_id(&self) -> i32 {
        self.usage_tool_card_id
    }
}