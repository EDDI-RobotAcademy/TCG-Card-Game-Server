use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NotifyOpponentRemoveFieldEnergySupportUsage {
    usage_support_card_number: i32,
    amount_to_remove: i32
}

impl NotifyOpponentRemoveFieldEnergySupportUsage {
    pub fn new(usage_support_card_number: i32, amount_to_remove: i32) -> Self {
        NotifyOpponentRemoveFieldEnergySupportUsage {
            usage_support_card_number,
            amount_to_remove,
        }
    }
    pub fn get_usage_support_card_number(&self) -> i32 { self.usage_support_card_number }
    pub fn get_amount_to_remove(&self) -> i32 { self.amount_to_remove }
}