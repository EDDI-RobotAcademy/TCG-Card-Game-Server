use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FindOpponentByAccountIdResponse {
    opponent_unique_id: i32,
}

impl FindOpponentByAccountIdResponse {
    pub fn new(opponent_unique_id: i32) -> Self {
        FindOpponentByAccountIdResponse { opponent_unique_id }
    }

    pub fn get_opponent_unique_id(&self) -> i32 {
        self.opponent_unique_id
    }
}
