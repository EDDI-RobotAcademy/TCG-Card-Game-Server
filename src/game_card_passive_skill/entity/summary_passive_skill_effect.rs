use crate::game_card_passive_skill::entity::passive_skill_casting_condition::PassiveSkillCastingCondition;
use crate::game_card_passive_skill::entity::passive_skill_type::PassiveSkillType;

#[derive(Debug, Clone, PartialEq)]
pub struct SummaryPassiveSkillEffect {
    passive_skill_type: PassiveSkillType,
    passive_skill_casting_condition: Vec<PassiveSkillCastingCondition>,
    skill_damage: i32,
}

impl SummaryPassiveSkillEffect {
    pub fn new(passive_skill_type: PassiveSkillType,
               passive_skill_casting_condition: Vec<PassiveSkillCastingCondition>,
               skill_damage: i32) -> Self {

        SummaryPassiveSkillEffect {
            passive_skill_type,
            passive_skill_casting_condition,
            skill_damage,
        }
    }

    pub fn get_passive_skill_type(&self) -> &PassiveSkillType {
        &self.passive_skill_type
    }
    pub fn get_passive_skill_casting_condition(&self) -> &Vec<PassiveSkillCastingCondition> { &self.passive_skill_casting_condition }
    pub fn get_skill_damage(&self) -> i32 {
        self.skill_damage
    }
}
