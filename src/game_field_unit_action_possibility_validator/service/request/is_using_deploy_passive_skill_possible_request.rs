use std::collections::HashMap;
use crate::common::card_attributes::card_race::card_race_enum::RaceEnum;
use crate::game_card_passive_skill::entity::passive_skill_casting_condition::PassiveSkillCastingCondition;
use crate::game_card_passive_skill::entity::passive_skill_type::PassiveSkillType;

#[derive(Debug)]
pub struct IsUsingDeployPassiveSkillPossibleRequest {
    account_unique_id: i32,
    field_unit_index: i32,
    passive_skill_index: i32,
    passive_skill_casting_condition: Vec<PassiveSkillCastingCondition>
}

impl IsUsingDeployPassiveSkillPossibleRequest {
    pub fn new(account_unique_id: i32,
               field_unit_index: i32,
               passive_skill_index: i32,
               passive_skill_casting_condition: Vec<PassiveSkillCastingCondition>

    ) -> Self {
        IsUsingDeployPassiveSkillPossibleRequest {
            account_unique_id,
            field_unit_index,
            passive_skill_index,
            passive_skill_casting_condition
        }
    }

    pub fn get_account_unique_id(&self) -> i32 { self.account_unique_id }
    pub fn get_field_unit_index(&self) -> i32 { self.field_unit_index }
    pub fn get_passive_skill_index(&self) -> i32 { self.passive_skill_index }
    pub fn get_passive_skill_casting_condition(self) -> Vec<PassiveSkillCastingCondition> { self.passive_skill_casting_condition }

}