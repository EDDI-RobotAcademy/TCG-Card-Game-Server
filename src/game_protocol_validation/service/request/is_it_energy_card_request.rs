#[derive(Debug)]
pub struct IsItEnergyCardRequest {
    energy_card_id: i32,
}

impl IsItEnergyCardRequest {
    pub fn new(energy_card_id: i32) -> Self {
        IsItEnergyCardRequest {
            energy_card_id
        }
    }

    pub fn get_energy_card_id(&self) -> i32 {
        self.energy_card_id
    }
}
