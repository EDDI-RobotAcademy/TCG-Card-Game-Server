use crate::common::card_attributes::card_race::card_race_enum::RaceEnum;
use crate::game_card_energy::entity::summary_energy_card_effect::SummaryEnergyCardEffect;

pub struct SummaryEnergyCardEffectResponse {
    race: RaceEnum,
    quantity: i32,
}

impl SummaryEnergyCardEffectResponse {
    pub fn new(race: RaceEnum, quantity: i32) -> Self {
        SummaryEnergyCardEffectResponse {
            race,
            quantity,
        }
    }

    pub fn get_race(&self) -> RaceEnum {
        self.race
    }

    pub fn get_quantity(&self) -> i32 { self.quantity }

    pub fn from_summary_energy_card_effect(summary_energy_card_effect: SummaryEnergyCardEffect) -> SummaryEnergyCardEffectResponse {
        SummaryEnergyCardEffectResponse::new(*summary_energy_card_effect.get_energy_card().get_race(),
                                             summary_energy_card_effect.get_energy_card().get_quantity())
    }
}