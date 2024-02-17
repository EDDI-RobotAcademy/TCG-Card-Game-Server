#[derive(Debug)]
pub struct NotifyOpponentYouUseEnergyCardRequest {
    opponent_unique_id: i32,
    usage_energy_card_id: i32,
    unit_card_index: i32,
    energy_race: i32,
    energy_count: i32,
}

impl NotifyOpponentYouUseEnergyCardRequest {
    pub fn new(opponent_unique_id: i32, usage_energy_card_id: i32, unit_card_index: i32, energy_race: i32, energy_count: i32) -> Self {
        NotifyOpponentYouUseEnergyCardRequest {
            opponent_unique_id,
            usage_energy_card_id,
            unit_card_index,
            energy_race,
            energy_count,
        }
    }

    pub fn get_opponent_unique_id(&self) -> i32 {
        self.opponent_unique_id
    }

    pub fn get_usage_energy_card_id(&self) -> i32 { self.usage_energy_card_id }

    pub fn get_unit_card_index(&self) -> i32 {
        self.unit_card_index
    }

    pub fn get_energy_race(&self) -> i32 { self.energy_race }

    pub fn get_energy_count(&self) -> i32 { self.energy_count }
}
