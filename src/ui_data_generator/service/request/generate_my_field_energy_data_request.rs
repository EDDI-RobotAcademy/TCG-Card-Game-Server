#[derive(Debug)]
pub struct GenerateMyFieldEnergyDataRequest {
    remaining_field_energy: i32
}

impl GenerateMyFieldEnergyDataRequest {
    pub fn new(remaining_field_energy: i32) -> Self {
        GenerateMyFieldEnergyDataRequest {
            remaining_field_energy,
        }
    }

    pub fn get_remaining_field_energy(&self) -> i32 { self.remaining_field_energy }
}