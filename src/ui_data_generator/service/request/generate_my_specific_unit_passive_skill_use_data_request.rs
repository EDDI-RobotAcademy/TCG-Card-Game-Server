#[derive(Debug)]
pub struct GenerateMySpecificUnitPassiveSkillUseDataRequest {
    attacker_unit_index: i32,
    target_unit_index: i32,
    passive_skill_index: i32
}

impl GenerateMySpecificUnitPassiveSkillUseDataRequest {
    pub fn new(attacker_unit_index: i32,
               target_unit_index: i32,
               passive_skill_index: i32) -> Self {
        GenerateMySpecificUnitPassiveSkillUseDataRequest {
            attacker_unit_index,
            target_unit_index,
            passive_skill_index,
        }
    }

    pub fn get_attacker_unit_index(&self) -> i32 { self.attacker_unit_index }

    pub fn get_target_unit_index(&self) -> i32 { self.target_unit_index }

    pub fn get_passive_skill_index(&self) -> i32 { self.passive_skill_index }
}