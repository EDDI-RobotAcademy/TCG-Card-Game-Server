use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CheckTurnActionResponse {
    has_already_taken_action: bool
}

impl CheckTurnActionResponse {
    pub fn new(has_already_taken_action: bool) -> Self {
        CheckTurnActionResponse { has_already_taken_action }
    }

    pub fn has_already_taken_action(&self) -> bool {
        self.has_already_taken_action
    }
}
