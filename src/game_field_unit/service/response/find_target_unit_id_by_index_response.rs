use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FindTargetUnitIdByIndexResponse {
    found_opponent_unit_id: i32
}

impl FindTargetUnitIdByIndexResponse {
    pub fn new(found_opponent_unit_id: i32) -> Self {
        FindTargetUnitIdByIndexResponse { found_opponent_unit_id }
    }

    pub fn get_found_opponent_unit_id(&self) -> i32 {
        self.found_opponent_unit_id
    }
}
