use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NotifyOpponentToEnhanceAttackPointToolUsage {
    unit_card_index: i32,
    usage_tool_card_id: i32,
}

impl NotifyOpponentToEnhanceAttackPointToolUsage {
    pub fn new(unit_card_index: i32, usage_tool_card_id: i32) -> Self {
        Self { unit_card_index, usage_tool_card_id }
    }

    pub fn get_unit_card_index(&self) -> i32 {
        self.unit_card_index
    }

    pub fn get_usage_tool_card_id(&self) -> i32 {
        self.usage_tool_card_id
    }
}