use crate::common::card_attributes::card_race::card_race_enum::RaceEnum;
use crate::game_card_energy::entity::summary_energy_card_effect::SummaryEnergyCardEffect;

pub struct SummaryEnergyCardEffectResponse {
    race: RaceEnum
}

impl SummaryEnergyCardEffectResponse {
    pub fn new(race: RaceEnum) -> Self {
        Self { race }
    }

    pub fn get_race(&self) -> RaceEnum {
        self.race
    }

    pub fn from_summary_energy_card_effect(summary_energy_card_effect: SummaryEnergyCardEffect) -> SummaryEnergyCardEffectResponse {
        SummaryEnergyCardEffectResponse::new(*summary_energy_card_effect.get_energy_card().get_race())
    }
}