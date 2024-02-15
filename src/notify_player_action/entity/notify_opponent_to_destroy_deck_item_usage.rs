use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NotifyOpponentToDestroyDeckItemUsage {
    usage_item_card_id: i32,
    will_be_lost_card: i32,
}

impl NotifyOpponentToDestroyDeckItemUsage {
    pub fn new(usage_item_card_id: i32, will_be_lost_card: i32) -> Self {
        NotifyOpponentToDestroyDeckItemUsage {
            usage_item_card_id,
            will_be_lost_card
        }
    }
    pub fn get_usage_item_card_id(&self) -> i32 { self.usage_item_card_id }
    pub fn get_damage_for_field_unit(&self) -> i32 { self.will_be_lost_card }
}