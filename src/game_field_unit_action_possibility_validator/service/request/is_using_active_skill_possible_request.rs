use std::collections::HashMap;
use crate::common::card_attributes::card_race::card_race_enum::RaceEnum;

#[derive(Debug)]
pub struct IsUsingActiveSkillPossibleRequest {
    account_unique_id: i32,
    field_unit_index: i32,
    skill_required_energy_map: HashMap<RaceEnum, i32>,
}

impl IsUsingActiveSkillPossibleRequest {
    pub fn new(account_unique_id: i32,
               field_unit_index: i32,
               skill_required_energy_map: HashMap<RaceEnum, i32>,) -> Self {
        IsUsingActiveSkillPossibleRequest {
            account_unique_id,
            field_unit_index,
            skill_required_energy_map
        }
    }

    pub fn get_account_unique_id(&self) -> i32 { self.account_unique_id }

    pub fn get_field_unit_index(&self) -> i32 { self.field_unit_index }

    pub fn get_skill_required_energy_map(&self) -> &HashMap<RaceEnum, i32> { &self.skill_required_energy_map }
}