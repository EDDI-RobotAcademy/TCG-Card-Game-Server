use crate::common::card_attributes::card_race::card_race_enum::RaceEnum;

pub struct EnergyCard {
    race: RaceEnum
}

impl EnergyCard {
    pub fn new(race: RaceEnum) -> Self {
        EnergyCard {
            race,
        }
    }

    pub fn get_race(&self) -> &RaceEnum {
        &self.race
    }
}
