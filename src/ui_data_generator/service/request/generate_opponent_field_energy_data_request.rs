#[derive(Debug)]
pub struct GenerateOpponentFieldEnergyDataRequest {
    remaining_field_energy: i32
}

impl GenerateOpponentFieldEnergyDataRequest {
    pub fn new(remaining_field_energy: i32) -> Self {
        GenerateOpponentFieldEnergyDataRequest {
            remaining_field_energy
        }
    }

    pub fn get_remaining_field_energy(&self) -> i32 { self.remaining_field_energy }
}