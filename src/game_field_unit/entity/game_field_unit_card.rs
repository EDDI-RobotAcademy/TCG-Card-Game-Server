use std::os::linux::raw::stat;
use crate::common::card_attributes::card_grade::card_grade_enum::GradeEnum;
use crate::common::card_attributes::card_race::card_race_enum::RaceEnum;
use crate::game_card_energy::entity::status_effect::StatusEffect;
use crate::game_field_unit::entity::attached_energy_map::AttachedEnergyMap;
use crate::game_field_unit::entity::extra_effect::ExtraEffect;
use crate::game_field_unit::entity::extra_status_effect::ExtraStatusEffect;
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
    has_first_passive_skill: bool,
    has_second_passive_skill: bool,
    has_third_passive_skill: bool,
    extra_status_effect_list: Vec<ExtraStatusEffect>,
    is_alive: bool,
}

impl GameFieldUnitCard {
    pub fn new(field_unit_card: i32,
               field_unit_race: RaceEnum,
               field_unit_grade: GradeEnum,
               unit_attack_point: i32,
               unit_health_point: i32,
               unit_attack_required_energy: i32,
               has_first_passive_skill: bool,
               has_second_passive_skill: bool,
               has_third_passive_skill: bool,
               is_alive: bool) -> GameFieldUnitCard {

        GameFieldUnitCard {
            field_unit_card,
            attached_energy_map: AttachedEnergyMap::new(),
            field_unit_race,
            field_unit_grade,
            unit_attack_point,
            unit_health_point: UnitHealthPoint::new(unit_health_point),
            unit_attack_required_energy,
            has_first_passive_skill,
            has_second_passive_skill,
            has_third_passive_skill,
            extra_status_effect_list: Vec::new(),
            is_alive
        }
    }

    pub fn get_card(&self) -> i32 {
        self.field_unit_card
    }

    pub fn get_attached_energy(&self) -> &AttachedEnergyMap {
        &self.attached_energy_map
    }

    pub fn get_unit_health_point(&self) -> &UnitHealthPoint {
        &self.unit_health_point
    }

    pub fn get_mut_unit_health_point(&mut self) -> &mut UnitHealthPoint {
        &mut self.unit_health_point
    }

    pub fn is_alive(&self) -> bool {
        self.is_alive
    }

    pub fn set_is_alive(&mut self, is_alive: bool) {
        self.is_alive = is_alive;
    }

    pub fn attach_energy(&mut self, race: RaceEnumValue, quantity: i32) {
        self.attached_energy_map.add_energy(race, quantity);
    }

    pub fn increase_max_health(&mut self, increase_point: i32) {
        self.unit_health_point.increase_max_health(increase_point);
    }

    pub fn apply_damage(&mut self, damage: i32) {
        let remaining_health = self.unit_health_point.get_current_health_point() - damage;
        let current_health = remaining_health.max(0);

        self.unit_health_point.set_current_health_point(current_health);
    }

    pub fn attach_special_energy(&mut self, race: RaceEnumValue, quantity: i32, status_effect_list: Vec<StatusEffect>) {
        self.attached_energy_map.add_energy(race, quantity);
        for status_effect in status_effect_list.iter().cloned() {
            let effect_number = status_effect.get_effect().to_i32();
            self.extra_status_effect_list.push(ExtraStatusEffect::new(
                ExtraEffect::from(effect_number),
                status_effect.get_status_duration_turn(),
                status_effect.get_effect_damage(),
                status_effect.get_reuse_turn()));
        }
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
            1,
            false,
            false,
            false,
            true);

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
            1,
            false,
            false,
            false,
            true);

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

    #[test]
    fn test_increase_max_health() {
        let mut game_field_unit_card = GameFieldUnitCard::new(
            5,
            RaceEnum::Chaos,
            GradeEnum::Hero,
            20,
            20,
            1,
            false,
            false,
            false,
            true);

        println!("game_field_unit_card: {:?}", game_field_unit_card);

        game_field_unit_card.increase_max_health(10);
        println!("game_field_unit_card: {:?}", game_field_unit_card);

        assert_eq!(game_field_unit_card.get_unit_health_point().get_max_health_point(), 30);
        assert_eq!(game_field_unit_card.get_unit_health_point().get_current_health_point(), 30);

        println!("Test passed: increase_max_health");
    }

    #[test]
    fn test_apply_damage() {
        let mut game_field_unit_card = GameFieldUnitCard::new(
            5,
            RaceEnum::Chaos,
            GradeEnum::Hero,
            20,
            20,
            1,
            false,
            false,
            false,
            true
        );

        println!("unit: {:?}", game_field_unit_card);
        game_field_unit_card.apply_damage(5);

        assert_eq!(
            game_field_unit_card.get_unit_health_point().get_current_health_point(),
            15
        );

        println!("unit: {:?}", game_field_unit_card);
        game_field_unit_card.apply_damage(20);

        assert_eq!(
            game_field_unit_card.get_unit_health_point().get_current_health_point(),
            0
        );

        println!("unit: {:?}", game_field_unit_card);
        println!("Test passed: apply_damage");
    }
}
