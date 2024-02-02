use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameDeckCardShuffledListResponse {
    deck_card_list: Vec<i32>,
}

impl GameDeckCardShuffledListResponse {
    pub fn new(deck_card_list: Vec<i32>) -> Self {
        GameDeckCardShuffledListResponse { deck_card_list }
    }
}
