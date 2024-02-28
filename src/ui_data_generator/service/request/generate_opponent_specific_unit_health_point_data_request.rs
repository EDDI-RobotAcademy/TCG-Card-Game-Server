#[derive(Debug)]
pub struct GenerateOpponentSpecificUnitHealthPointDataRequest {
    opponent_unit_index: i32,
    opponent_unit_updated_health_point: i32,
}

impl GenerateOpponentSpecificUnitHealthPointDataRequest {
    pub fn new(opponent_unit_index: i32,
               opponent_unit_updated_health_point: i32,) -> Self {
        GenerateOpponentSpecificUnitHealthPointDataRequest {
            opponent_unit_index,
            opponent_unit_updated_health_point
        }
    }

    pub fn get_opponent_unit_index(&self) -> i32 { self.opponent_unit_index }

    pub fn get_opponent_unit_updated_health_point(&self) -> i32 { self.opponent_unit_updated_health_point }
}