use crate::common::card_attributes::card_grade::card_grade_enum::GradeEnum;
use crate::common::card_attributes::card_race::card_race_enum::RaceEnum;
use crate::game_field_unit::entity::attached_energy_map::AttachedEnergyMap;
use crate::game_field_unit::entity::race_enum_value::RaceEnumValue;
use crate::game_field_unit::entity::unit_health_point::UnitHealthPoint;

#[derive(Debug, Clone)]
pub struct GameFieldUnitCard {
    field_unit_card: i32,
    attached_energy_map: AttachedEnergyMap,
    field_unit_race: RaceEnum,
    field_unit_grade: GradeEnum,
    unit_attack_point: i32,
    unit_health_point: UnitHealthPoint,
    unit_attack_required_energy: i32,
}

impl GameFieldUnitCard {
    pub fn new(field_unit_card: i32,
               field_unit_race: RaceEnum,
               field_unit_grade: GradeEnum,
               unit_attack_point: i32,
               unit_health_point: i32,
               unit_attack_required_energy: i32) -> GameFieldUnitCard {
        GameFieldUnitCard {
            field_unit_card,
            attached_energy_map: AttachedEnergyMap::new(),
            field_unit_race,
            field_unit_grade,
            unit_attack_point,
            unit_health_point: UnitHealthPoint::new(unit_health_point),
            unit_attack_required_energy
        }
    }

    pub fn get_card(&self) -> i32 {
        self.field_unit_card
    }

    pub fn get_attached_energy(&self) -> &AttachedEnergyMap {
        &self.attached_energy_map
    }

    pub fn attach_energy(&mut self, race: RaceEnumValue, quantity: i32) {
        self.attached_energy_map.add_energy(race, quantity);
    }
    pub fn get_unit_health_point(&self) -> &UnitHealthPoint {
        &self.unit_health_point
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_game_field_unit_card_creation() {
        let game_field_unit_card = GameFieldUnitCard::new(
            5,
            RaceEnum::Chaos,
            GradeEnum::Hero,
            20,
            20,
            1);
        assert_eq!(game_field_unit_card.get_card(), 5);
        println!("Test passed: FieldUnit creation and getter");
    }

    #[test]
    fn test_attach_energy() {
        let mut game_field_unit_card = GameFieldUnitCard::new(
            5,
            RaceEnum::Chaos,
            GradeEnum::Hero,
            20,
            20,
            1);
        assert_eq!(game_field_unit_card.get_card(), 5);
        assert_eq!(game_field_unit_card.get_attached_energy().get_all_energy().len(), 0);

        game_field_unit_card.attach_energy(RaceEnumValue::Undead, 3);
        assert_eq!(game_field_unit_card.get_attached_energy().get_all_energy().len(), 1);
        assert_eq!(*game_field_unit_card.get_attached_energy().get_energy_quantity(&RaceEnumValue::Undead).unwrap(), 3);

        game_field_unit_card.attach_energy(RaceEnumValue::Human, 5);
        assert_eq!(game_field_unit_card.get_attached_energy().get_all_energy().len(), 2);
        assert_eq!(*game_field_unit_card.get_attached_energy().get_energy_quantity(&RaceEnumValue::Human).unwrap(), 5);

        println!("{:?}", game_field_unit_card);

        println!("Test passed: FieldUnit creation, getter, attach_energy, and print_state");
    }
}
