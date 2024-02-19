use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NotifyOpponentToEnergyUsage {
    usage_energy_card_id: i32,
    unit_card_index: i32,
    energy_race: i32,
    energy_count: i32
}

impl NotifyOpponentToEnergyUsage {
    pub fn new(usage_energy_card_id: i32, unit_card_index: i32, energy_race: i32, energy_count: i32) -> Self {
        Self { unit_card_index, usage_energy_card_id, energy_race, energy_count }
    }

    pub fn get_unit_card_index(&self) -> i32 {
        self.unit_card_index
    }

    pub fn get_usage_energy_card_id(&self) -> i32 {
        self.usage_energy_card_id
    }

    pub fn get_energy_race(&self) -> i32 { self.energy_race }

    pub fn get_energy_count(&self) -> i32 { self.energy_count }
}