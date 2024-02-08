use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameDeckCardShuffleResponse {
    is_success: bool,
}

impl GameDeckCardShuffleResponse {
    pub fn new(is_success: bool) -> Self {
        GameDeckCardShuffleResponse { is_success }
    }
    pub fn is_success(&self) -> bool { self.is_success }
}
