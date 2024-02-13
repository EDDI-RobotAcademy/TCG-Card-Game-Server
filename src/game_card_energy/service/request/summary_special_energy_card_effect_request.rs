use crate::game_card_energy::entity::status_effect::StatusEffect;

#[derive(Clone)]
pub struct SummarySpecialEnergyCardEffectRequest {
    special_energy_card_id: i32,
}

impl SummarySpecialEnergyCardEffectRequest {
    pub fn new(special_energy_card_id: i32) -> Self {
        SummarySpecialEnergyCardEffectRequest {
            special_energy_card_id,
        }
    }

    pub fn get_special_energy_card_id(&self) -> i32 {
        self.special_energy_card_id
    }
}
