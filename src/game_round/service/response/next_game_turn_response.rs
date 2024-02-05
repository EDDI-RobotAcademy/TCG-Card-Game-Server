use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NextGameRoundResponse {
    next_turn: i32,
}

impl NextGameRoundResponse {
    pub fn new(next_turn: i32) -> Self {
        NextGameRoundResponse { next_turn }
    }
}
