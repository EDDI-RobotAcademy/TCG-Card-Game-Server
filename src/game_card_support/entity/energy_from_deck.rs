use crate::common::card_attributes::card_race::card_race_enum::RaceEnum;
use crate::game_card_support::entity::game_card_support_effect::GameCardSupportEffect;

#[derive(Debug, PartialEq)]
pub struct EnergyFromDeck {
    race: RaceEnum,
    energy_count: i32,
}

impl EnergyFromDeck {
    pub fn new(race: RaceEnum, energy_count: i32) -> Self {
        EnergyFromDeck {
            race,
            energy_count,
        }
    }

    pub fn get_race(&self) -> &RaceEnum {
        &self.race
    }

    pub fn get_energy_count(&self) -> i32 {
        self.energy_count
    }
}