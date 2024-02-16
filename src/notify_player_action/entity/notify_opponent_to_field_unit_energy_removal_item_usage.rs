use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NotifyOpponentToFieldUnitEnergyRemovalItemUsage {
    usage_item_card_id: i32,
    decreased_energy_quantity: i32
}

impl NotifyOpponentToFieldUnitEnergyRemovalItemUsage {
    pub fn new(usage_item_card_id: i32, decreased_energy_quantity: i32) -> Self {
        Self { usage_item_card_id, decreased_energy_quantity }
    }
    pub fn get_usage_item_card_id(&self) -> i32 {
        self.usage_item_card_id
    }

    pub fn get_decreased_energy_quantity(&self) -> i32 {
        self.decreased_energy_quantity
    }
}