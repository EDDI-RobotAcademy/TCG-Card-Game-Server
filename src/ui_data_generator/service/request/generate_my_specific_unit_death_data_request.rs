#[derive(Debug)]
pub struct GenerateMySpecificUnitDeathDataRequest {
    dead_unit_index: i32,
}

impl GenerateMySpecificUnitDeathDataRequest {
    pub fn new(dead_unit_index: i32,) -> Self {
        GenerateMySpecificUnitDeathDataRequest {
            dead_unit_index,
        }
    }

    pub fn get_dead_unit_index(&self) -> i32 { self.dead_unit_index }
}