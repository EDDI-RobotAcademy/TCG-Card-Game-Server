use crate::common::card_attributes::card_grade::card_grade_enum::GradeEnum;
use crate::common::card_attributes::card_race::card_race_enum::RaceEnum;
use crate::game_card_energy::entity::summary_energy_card_effect::SummaryEnergyCardEffect;
use crate::game_card_item::entity::field_energy_addition_calculator::FieldEnergyAdditionCalculator;
use crate::game_card_item::entity::game_card_item_effect::GameCardItemEffect;

pub struct SummaryItemCardEffectResponse {
    // TODO: 향후 VO 배치 필요
    required_energy_race: RaceEnum,
    required_energy_count: i32,
    alternatives_damage: i32,
    apply_target_minimum_grade: GradeEnum,
    field_energy_addition_calculator: FieldEnergyAdditionCalculator,
    catastrophic_damage_for_field_unit: i32,
    catastrophic_damage_for_main_character: i32,
    will_be_lost_deck_card_count: i32,
    target_count_that_can_be_damaged: i32,
    unit_list_that_can_be_sacrificed: Vec<i32>,
}

impl SummaryItemCardEffectResponse {
    pub fn new(
        required_energy_race: RaceEnum,
        required_energy_count: i32,
        alternatives_damage: i32,
        apply_target_minimum_grade: GradeEnum,
        field_energy_addition_calculator: FieldEnergyAdditionCalculator,
        catastrophic_damage_for_field_unit: i32,
        catastrophic_damage_for_main_character: i32,
        will_be_lost_deck_card_count: i32,
        target_count_that_can_be_damaged: i32,
        unit_list_that_can_be_sacrificed: Vec<i32>,) -> Self {

            Self {
                required_energy_race,
                required_energy_count,
                alternatives_damage,
                apply_target_minimum_grade,
                field_energy_addition_calculator,
                catastrophic_damage_for_field_unit,
                catastrophic_damage_for_main_character,
                will_be_lost_deck_card_count,
                target_count_that_can_be_damaged,
                unit_list_that_can_be_sacrificed,}
    }

    pub fn get_required_energy_race(&self) -> RaceEnum {
        self.required_energy_race
    }

    pub fn get_required_energy_count(&self) -> i32 {
        self.required_energy_count
    }

    pub fn get_alternatives_damage(&self) -> i32 {
        self.alternatives_damage
    }

    pub fn get_apply_target_minimum_grade(&self) -> GradeEnum {
        self.apply_target_minimum_grade
    }

    pub fn get_field_energy_addition_calculator(&mut self) -> FieldEnergyAdditionCalculator { self.field_energy_addition_calculator }

    pub fn get_catastrophic_damage_for_field_unit(&self) -> i32 { self.catastrophic_damage_for_field_unit }

    pub fn get_catastrophic_damage_for_main_character(&self) -> i32 { self.catastrophic_damage_for_main_character}

    pub fn get_will_be_lost_deck_card_count(&self) -> i32 { self.will_be_lost_deck_card_count }

    pub fn get_target_count_that_can_be_damaged(&self) -> i32 { self.target_count_that_can_be_damaged }

    pub fn get_unit_list_that_can_be_sacrificed(&self) -> Vec<i32> { self.unit_list_that_can_be_sacrificed.clone() }

    pub fn from_summary_item_card_effect(game_card_item_effect: GameCardItemEffect) -> SummaryItemCardEffectResponse {
        let required_energy = game_card_item_effect.get_required_energy();
        SummaryItemCardEffectResponse::new(
            *required_energy.get_required_energy_race(),
            required_energy.get_required_energy_count(),
            game_card_item_effect.get_alternatives_damage(),
            game_card_item_effect.get_apply_target_minimum_grade(),
            game_card_item_effect.get_field_energy_addition_calculator(),
            game_card_item_effect.get_catastrophic_damage_for_field_unit(),
            game_card_item_effect.get_catastrophic_damage_for_main_character(),
            game_card_item_effect.get_will_be_lost_deck_card_count(),
            game_card_item_effect.get_target_count_that_can_be_damaged(),
            game_card_item_effect.get_unit_list_that_can_be_sacrificed(),)
    }
}