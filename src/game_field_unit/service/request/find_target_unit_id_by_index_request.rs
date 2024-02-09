#[derive(Debug)]
pub struct FindTargetUnitIdByIndexRequest {
    opponent_unique_id: i32,
    opponent_target_unit_index: i32
}

impl FindTargetUnitIdByIndexRequest {
    pub fn new(opponent_unique_id: i32, opponent_target_unit_index: i32) -> Self {
        FindTargetUnitIdByIndexRequest {
            opponent_unique_id,
            opponent_target_unit_index
        }
    }

    pub fn get_opponent_unique_id(&self) -> i32 {
        self.opponent_unique_id
    }

    pub fn get_opponent_target_unit_index(&self) -> i32 {
        self.opponent_target_unit_index
    }
}
