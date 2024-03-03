use crate::game_card_passive_skill::entity::passive_skill_casting_condition::PassiveSkillCastingCondition;
use crate::game_card_passive_skill::entity::passive_skill_type::PassiveSkillType;
use crate::game_card_passive_skill::entity::summary_passive_skill_effect::SummaryPassiveSkillEffect;

pub struct SummaryPassiveSkillEffectByIndexResponse {
    passive_skill_type: PassiveSkillType,
    passive_skill_casting_condition: Vec<PassiveSkillCastingCondition>,
    skill_damage: i32,
}

impl SummaryPassiveSkillEffectByIndexResponse {
    pub fn new(
        passive_skill_type: PassiveSkillType,
        passive_skill_casting_condition: Vec<PassiveSkillCastingCondition>,
        skill_damage: i32,) -> Self {

        Self {
            passive_skill_type,
            passive_skill_casting_condition,
            skill_damage }
    }

    pub fn get_passive_skill_type(&self) -> &PassiveSkillType {
        &self.passive_skill_type
    }
    pub fn get_passive_skill_casting_condition(&self) -> &Vec<PassiveSkillCastingCondition> { &self.passive_skill_casting_condition }

    pub fn get_skill_damage(&self) -> i32 {
        self.skill_damage
    }

    pub fn from_summary_passive_skill_effect(summary_passive_skill_effect: SummaryPassiveSkillEffect) -> SummaryPassiveSkillEffectByIndexResponse {

        SummaryPassiveSkillEffectByIndexResponse::new(
            summary_passive_skill_effect.get_passive_skill_type().clone(),
            summary_passive_skill_effect.clone().get_passive_skill_casting_condition(),
            summary_passive_skill_effect.get_skill_damage())
    }

}