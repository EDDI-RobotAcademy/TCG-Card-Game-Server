use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NotifyOpponentToFieldEnergyUsage {
    unit_card_index: i32,
    energy_race: i32,
    energy_count: i32,
    current_unit_energy_count: i32,
    remaining_field_energy: i32
}

impl NotifyOpponentToFieldEnergyUsage {
    pub fn new(unit_card_index: i32, energy_race: i32, energy_count: i32, current_unit_energy_count: i32, remaining_field_energy: i32) -> Self {
        Self { unit_card_index, energy_race, energy_count, current_unit_energy_count, remaining_field_energy }
    }

    pub fn get_unit_card_index(&self) -> i32 {
        self.unit_card_index
    }

    pub fn get_energy_race(&self) -> i32 { self.energy_race }

    pub fn get_energy_count(&self) -> i32 { self.energy_count }

    pub fn get_current_unit_energy_count(&self) -> i32 { self.current_unit_energy_count }

    pub fn get_remaining_field_energy(&self) -> i32 { self.remaining_field_energy }
}