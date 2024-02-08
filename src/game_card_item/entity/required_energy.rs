use crate::common::card_attributes::card_grade::card_grade_enum::GradeEnum;
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_required_energy() {
        let race_enum = RaceEnum::Human;
        let required_energy = RequiredEnergy::new(race_enum, 1);

        assert_eq!(required_energy.get_required_energy_race(), &race_enum);
        assert_eq!(required_energy.get_required_energy_count(), 1);
        println!("required_energy: {:?}", required_energy)
    }
}
