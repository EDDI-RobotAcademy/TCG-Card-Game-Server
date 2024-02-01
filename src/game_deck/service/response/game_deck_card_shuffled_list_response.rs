use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameDeckCardShuffledListResponse {
    is_success: bool,
}

impl GameDeckCardShuffledListResponse {
    pub fn new(is_success: bool) -> Self {
        GameDeckCardShuffledListResponse { is_success }
    }
}
