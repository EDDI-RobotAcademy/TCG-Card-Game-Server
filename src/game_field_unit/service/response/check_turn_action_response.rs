use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CheckTurnActionResponse {
    turn_action: bool
}

impl CheckTurnActionResponse {
    pub fn new(turn_action: bool) -> Self {
        CheckTurnActionResponse { turn_action }
    }

    pub fn turn_action(&self) -> bool {
        self.turn_action
    }
}
