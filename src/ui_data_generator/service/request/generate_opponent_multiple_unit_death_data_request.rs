#[derive(Debug)]
pub struct GenerateOpponentMultipleUnitDeathDataRequest {
    opponent_dead_unit_index_list: Vec<i32>,
}

impl GenerateOpponentMultipleUnitDeathDataRequest {
    pub fn new(opponent_dead_unit_index_list: Vec<i32>,) -> Self {
        GenerateOpponentMultipleUnitDeathDataRequest {
            opponent_dead_unit_index_list,
        }
    }

    pub fn get_opponent_dead_unit_index_list(&self) -> &Vec<i32> { &self.opponent_dead_unit_index_list }
}