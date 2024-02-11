use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NotifyOpponentToInstantDeathItemUsage {
    unit_card_index: i32,
    usage_item_card_id: i32,
}

impl NotifyOpponentToInstantDeathItemUsage {
    pub fn new(unit_card_index: i32, usage_item_card_id: i32) -> Self {
        Self { unit_card_index, usage_item_card_id }
    }

    pub fn get_unit_card_index(&self) -> i32 {
        self.unit_card_index
    }

    pub fn get_usage_energy_card_id(&self) -> i32 {
        self.usage_item_card_id
    }
}