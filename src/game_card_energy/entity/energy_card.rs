use crate::common::card_attributes::card_race::card_race_enum::RaceEnum;

pub struct EnergyCard {
    race: RaceEnum,
    quantity: i32,
}

impl EnergyCard {
    pub fn new(race: RaceEnum) -> Self {
        EnergyCard {
            race,
            quantity: 1
        }
    }

    pub fn get_race(&self) -> &RaceEnum {
        &self.race
    }

    pub fn get_quantity(&self) -> i32 { self.quantity }
}
