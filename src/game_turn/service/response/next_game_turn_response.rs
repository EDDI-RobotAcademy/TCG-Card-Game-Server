use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NextGameTurnResponse {
    next_turn: i32,
}

impl NextGameTurnResponse {
    pub fn new(next_turn: i32) -> Self {
        NextGameTurnResponse { next_turn }
    }
}
