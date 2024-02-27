#[derive(Debug)]
pub struct NoticeAddFieldEnergyRequest {
    opponent_unique_id: i32,
    remaining_field_energy: i32
}

impl NoticeAddFieldEnergyRequest {
    pub fn new(opponent_unique_id: i32,
               remaining_field_energy: i32) -> Self {
        NoticeAddFieldEnergyRequest {
            opponent_unique_id,
            remaining_field_energy
        }
    }

    pub fn get_opponent_unique_id(&self) -> i32 { self.opponent_unique_id }

    pub fn get_remaining_field_energy(&self) -> i32 { self.remaining_field_energy }
}