use crate::common::card_attributes::card_grade::card_grade_enum::GradeEnum;
use crate::common::card_attributes::card_race::card_race_enum::RaceEnum;
use crate::game_card_active_skill::entity::skill_type::SkillType;
use crate::game_card_active_skill::entity::summary_active_skill_effect::SummaryActiveSkillEffect;
use crate::game_card_energy::entity::summary_energy_card_effect::SummaryEnergyCardEffect;
use crate::game_card_item::entity::field_energy_addition_calculator::FieldEnergyAdditionCalculator;
use crate::game_card_item::entity::game_card_item_effect::GameCardItemEffect;

pub struct SummaryActiveSkillEffectResponse {
    skill_type: SkillType,
    skill_damage: i32,
}

impl SummaryActiveSkillEffectResponse {
    pub fn new(
        skill_type: SkillType,
        skill_damage: i32,) -> Self {

        Self {
            skill_type,
            skill_damage }
    }

    pub fn get_skill_type(&self) -> &SkillType {
        &self.skill_type
    }

    pub fn get_skill_damage(&self) -> i32 {
        self.skill_damage
    }

    pub fn from_summary_active_skill_effect(summary_active_skill_effect: SummaryActiveSkillEffect) -> SummaryActiveSkillEffectResponse {

        SummaryActiveSkillEffectResponse::new(
            summary_active_skill_effect.get_skill_type().clone(),
            summary_active_skill_effect.get_skill_damage())
    }

}