#[derive(Debug)]
pub struct GenerateOpponentMultipleUnitHealthPointDataRequest {
    opponent_unit_health_point_tuple_list: Vec<(i32, i32)>,
}

impl GenerateOpponentMultipleUnitHealthPointDataRequest {
    pub fn new(opponent_unit_health_point_tuple_list: Vec<(i32, i32)>,) -> Self {
        GenerateOpponentMultipleUnitHealthPointDataRequest {
            opponent_unit_health_point_tuple_list,
        }
    }

    pub fn get_opponent_unit_health_point_tuple_list(&self) -> Vec<(i32, i32)> {
        self.opponent_unit_health_point_tuple_list.clone()
    }
}