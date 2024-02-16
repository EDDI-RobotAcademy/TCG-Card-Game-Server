use crate::common::card_attributes::card_race::card_race_enum::RaceEnum;
use crate::game_card_energy::entity::status_effect::StatusEffect;
use crate::game_card_energy::entity::summary_energy_card_effect::SummaryEnergyCardEffect;

pub struct SummarySpecialEnergyCardEffectResponse {
    race: RaceEnum,
    quantity: i32,
    status_effect_list: Vec<StatusEffect>
}

impl SummarySpecialEnergyCardEffectResponse {
    pub fn new(race: RaceEnum, quantity: i32, status_effect_list: Vec<StatusEffect>) -> Self {
        Self { race, quantity, status_effect_list }
    }

    pub fn get_race(&self) -> RaceEnum {
        self.race
    }

    pub fn get_quantity(&self) -> i32 { self.quantity }

    pub fn get_status_effect_list(&self) -> &Vec<StatusEffect> {
        &self.status_effect_list
    }

    pub fn from_summary_special_energy_card_effect(summary_energy_card_effect: SummaryEnergyCardEffect) -> SummarySpecialEnergyCardEffectResponse {
        SummarySpecialEnergyCardEffectResponse::new(
            *summary_energy_card_effect.get_energy_card().get_race(),
            summary_energy_card_effect.get_energy_card().get_quantity(),
            summary_energy_card_effect.get_status_effects().clone())
    }
}