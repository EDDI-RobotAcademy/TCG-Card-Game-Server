#[derive(Debug)]
pub struct GenerateMySpecificUnitBasicAttackDataRequest {
    attacker_unit_index: i32,
    target_unit_index: i32,
}

impl GenerateMySpecificUnitBasicAttackDataRequest {
    pub fn new(attacker_unit_index: i32,
               target_unit_index: i32,) -> Self {
        GenerateMySpecificUnitBasicAttackDataRequest {
            attacker_unit_index,
            target_unit_index
        }
    }

    pub fn get_attacker_unit_index(&self) -> i32 { self.attacker_unit_index }

    pub fn get_target_unit_index(&self) -> i32 { self.target_unit_index }
}