use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NotifyOpponentSearchSupportUsage {
    usage_support_card_id: i32,
    found_card_count: i32,
}

impl NotifyOpponentSearchSupportUsage {
    pub fn new(usage_support_card_id: i32, found_card_count: i32) -> Self {
        NotifyOpponentSearchSupportUsage {
            usage_support_card_id,
            found_card_count
        }
    }
    pub fn get_usage_support_card_id(&self) -> i32 { self.usage_support_card_id }
    pub fn get_found_card_count(&self) -> i32 { self.found_card_count }
}