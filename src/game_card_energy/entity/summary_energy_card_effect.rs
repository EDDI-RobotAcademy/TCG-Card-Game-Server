use crate::common::card_attributes::card_race::card_race_enum::RaceEnum;
use crate::game_card_energy::entity::energy_card::EnergyCard;

pub struct SummaryEnergyCardEffect {
    energy_card: EnergyCard
}

impl SummaryEnergyCardEffect {
    pub fn new(race: RaceEnum) -> Self {
        SummaryEnergyCardEffect {
            energy_card: EnergyCard::new(race),
        }
    }

    pub fn get_energy_card(&self) -> &EnergyCard {
        &self.energy_card
    }
}
