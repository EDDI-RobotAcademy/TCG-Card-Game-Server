#[derive(Debug)]
pub struct GenerateInstantDeathOfYourSpecificUnitDataRequest {
    dead_unit_index: i32,
}

impl GenerateInstantDeathOfYourSpecificUnitDataRequest {
    pub fn new(dead_unit_index: i32,) -> Self {
        GenerateInstantDeathOfYourSpecificUnitDataRequest {
            dead_unit_index,
        }
    }

    pub fn get_dead_unit_index(&self) -> i32 { self.dead_unit_index }
}