use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UseGameHandEnergyCardResponse {
    found_energy_card_id: i32,
}

impl UseGameHandEnergyCardResponse {
    pub fn new(found_energy_card_id: i32) -> Self {
        UseGameHandEnergyCardResponse { found_energy_card_id }
    }

    pub fn get_found_energy_card_id(&self) -> i32 {
        self.found_energy_card_id
    }
}
