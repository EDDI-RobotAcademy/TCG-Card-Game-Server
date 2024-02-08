use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameDeckStartCardListResponse {
    hand_card_list: Vec<i32>,
}

impl GameDeckStartCardListResponse {
    pub fn new(hand_card_list: Vec<i32>) -> Self {
        GameDeckStartCardListResponse { hand_card_list }
    }
}
