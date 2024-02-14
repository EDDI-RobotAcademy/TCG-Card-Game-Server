use crate::common::card_attributes::card_grade::card_grade_enum::GradeEnum;
use crate::common::card_attributes::card_race::card_race_enum::RaceEnum;
use crate::game_card_item::entity::field_energy_addition_calculator::FieldEnergyAdditionCalculator;
use crate::game_card_item::entity::required_energy::RequiredEnergy;
use crate::game_card_support::entity::energy_from_deck::EnergyFromDeck;

#[derive(PartialEq, Debug)]
pub struct GameCardItemEffect {
    required_energy: RequiredEnergy,
    alternatives_damage: i32,
    apply_target_minimum_grade: GradeEnum,
    field_energy_calculator: FieldEnergyAdditionCalculator,
    catastrophic_damage_for_field_unit: i32,
    catastrophic_damage_for_main_character: i32,
    will_be_lost_deck_card_count: i32,
}

impl GameCardItemEffect {
    pub fn new(required_energy_race: RaceEnum,
               required_energy_count: i32,
               alternatives_damage: i32,
               apply_target_minimum_grade: GradeEnum) -> Self {

        GameCardItemEffect {
            required_energy: RequiredEnergy::new(required_energy_race, required_energy_count),
            alternatives_damage,
            apply_target_minimum_grade,
            field_energy_calculator: FieldEnergyAdditionCalculator::new(-1),
            catastrophic_damage_for_field_unit: -1,
            catastrophic_damage_for_main_character: -1,
            will_be_lost_deck_card_count: -1,
        }
    }

    pub fn set_alternatives_damage(&mut self, alternatives_damage: i32) {
        self.alternatives_damage = alternatives_damage;
    }

    pub fn get_required_energy(&self) -> &RequiredEnergy {
        &self.required_energy
    }

    pub fn get_alternatives_damage(&self) -> i32 {
        self.alternatives_damage
    }

    pub fn get_apply_target_minimum_grade(&self) -> GradeEnum {
        self.apply_target_minimum_grade
    }

    pub fn set_field_energy_addition_calculator(&mut self, divider: i32) {
        self.field_energy_calculator.init_calculator_by_info(divider);
    }

    pub fn get_field_energy_addition_calculator(&self) -> FieldEnergyAdditionCalculator {
        self.field_energy_calculator
    }

    pub fn set_catastrophic_damage_for_field_unit(&mut self, damage_for_field_unit: i32) {
        self.catastrophic_damage_for_field_unit = damage_for_field_unit;
    }

    pub fn get_catastrophic_damage_for_field_unit(&self) -> i32 {
        self.catastrophic_damage_for_field_unit
    }

    pub fn set_catastrophic_damage_for_main_character(&mut self, damage_for_main_character: i32) {
        self.catastrophic_damage_for_main_character = damage_for_main_character;
    }

    pub fn get_catastrophic_damage_for_main_character(&self) -> i32 {
        self.catastrophic_damage_for_main_character
    }

    pub fn set_will_be_lost_deck_card_count(&mut self, will_be_lost_deck_card_count: i32) {
        self.will_be_lost_deck_card_count = will_be_lost_deck_card_count;
    }

    pub fn get_will_be_lost_deck_card_count(&self) -> i32 {
        self.will_be_lost_deck_card_count
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_game_card_item_effect() {
        let mut game_card_item_effect = GameCardItemEffect::new(
            RaceEnum::Human, 5, 30, GradeEnum::Legend);
        println!("Energy From Deck: {:?}", game_card_item_effect);

        assert_eq!(game_card_item_effect.get_alternatives_damage(), 30);
        assert_eq!(game_card_item_effect.get_required_energy().get_required_energy_race(), &RaceEnum::Human);
        assert_eq!(game_card_item_effect.get_required_energy().get_required_energy_count(), 5);
        assert_eq!(game_card_item_effect.get_apply_target_minimum_grade(), GradeEnum::Legend);

        // divider setting
        let divider = 5;
        game_card_item_effect.set_field_energy_addition_calculator(divider);

        let calculation_result = game_card_item_effect
            .get_field_energy_addition_calculator().calculation_of_field_energy_increment(23);

        assert_eq!(calculation_result, 4);

        game_card_item_effect.set_catastrophic_damage_for_field_unit(10);
        game_card_item_effect.set_catastrophic_damage_for_main_character(10);
        game_card_item_effect.set_will_be_lost_deck_card_count(1);

        println!("Energy From Deck: {:?}", game_card_item_effect);
    }
}
