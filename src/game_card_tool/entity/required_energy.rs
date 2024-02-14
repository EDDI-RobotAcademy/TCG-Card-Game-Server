use crate::common::card_attributes::card_race::card_race_enum::RaceEnum;

#[derive(Debug, PartialEq)]
pub struct RequiredEnergy {
    required_energy_race: RaceEnum,
    required_energy_count: i32,
}

impl RequiredEnergy {
    pub fn new(required_energy_race: RaceEnum, required_energy_count: i32) -> Self {
        RequiredEnergy {
            required_energy_race,
            required_energy_count,
        }
    }

    pub fn get_required_energy_race(&self) -> &RaceEnum {
        &self.required_energy_race
    }

    pub fn get_required_energy_count(&self) -> i32 {
        self.required_energy_count
    }
}