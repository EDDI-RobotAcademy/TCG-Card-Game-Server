use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameRoundResponse {
    is_success: bool,
}

impl GameRoundResponse {
    pub fn new(is_success: bool) -> Self {
        GameRoundResponse { is_success }
    }
}
