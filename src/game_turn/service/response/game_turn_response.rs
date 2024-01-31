use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameTurnResponse {
    is_success: bool,
}

impl GameTurnResponse {
    pub fn new(is_success: bool) -> Self {
        GameTurnResponse { is_success }
    }
}
