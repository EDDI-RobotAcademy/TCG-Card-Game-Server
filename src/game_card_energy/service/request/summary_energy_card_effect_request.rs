#[derive(Clone)]
pub struct SummaryEnergyCardEffectRequest {
    energy_card_id: i32,
}

impl SummaryEnergyCardEffectRequest {
    pub fn new(energy_card_id: i32) -> Self {
        SummaryEnergyCardEffectRequest {
            energy_card_id
        }
    }

    pub fn get_energy_card_id(&self) -> i32 {
        self.energy_card_id
    }
}
