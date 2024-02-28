#[derive(Debug)]
pub struct GenerateOpponentSpecificUnitDeathDataRequest {
    dead_unit_index: i32,
}

impl GenerateOpponentSpecificUnitDeathDataRequest {
    pub fn new(dead_unit_index: i32,) -> Self {
        GenerateOpponentSpecificUnitDeathDataRequest {
            dead_unit_index,
        }
    }

    pub fn get_dead_unit_index(&self) -> i32 { self.dead_unit_index }
}