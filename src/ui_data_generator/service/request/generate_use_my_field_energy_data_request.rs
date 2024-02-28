#[derive(Debug)]
pub struct GenerateUseMyFieldEnergyDataRequest {
    remaining_field_energy: i32
}

impl GenerateUseMyFieldEnergyDataRequest {
    pub fn new(remaining_field_energy: i32) -> Self {
        GenerateUseMyFieldEnergyDataRequest {
            remaining_field_energy,
        }
    }

    pub fn get_remaining_field_energy(&self) -> i32 { self.remaining_field_energy }
}