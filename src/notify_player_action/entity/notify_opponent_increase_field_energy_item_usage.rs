use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NotifyOpponentIncreaseFieldEnergyItemUsage {
    usage_item_card_number: i32,
    increased_amount: i32
}

impl NotifyOpponentIncreaseFieldEnergyItemUsage {
    pub fn new(usage_item_card_number: i32, increased_amount: i32) -> Self {
       NotifyOpponentIncreaseFieldEnergyItemUsage {
           usage_item_card_number,
           increased_amount,
       }
    }
    pub fn get_usage_item_card_number(&self) -> i32 { self.usage_item_card_number }
    pub fn get_increased_amount(&self) -> i32 { self.increased_amount }
}