use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NotifyOpponentToEnergyBoost {
    unit_card_index: i32,
    usage_energy_card_id: i32,
    boosting_energy_card_id_list: Vec<i32>
}

impl NotifyOpponentToEnergyBoost {
    pub fn new(unit_card_index: i32, usage_energy_card_id: i32, boosting_energy_card_id_list: Vec<i32>) -> Self {
        Self { unit_card_index, usage_energy_card_id, boosting_energy_card_id_list }
    }

    pub fn get_unit_card_index(&self) -> i32 {
        self.unit_card_index
    }

    pub fn get_usage_energy_card_id(&self) -> i32 {
        self.usage_energy_card_id
    }

    pub fn boosting_energy_card_id_list(&self) -> &Vec<i32> {
        &self.boosting_energy_card_id_list
    }
}