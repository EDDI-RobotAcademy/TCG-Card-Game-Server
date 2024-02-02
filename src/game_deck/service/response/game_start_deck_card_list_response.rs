use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameStartDeckCardListResponse {
    draw_card_list: Vec<i32>,
    deck_card_list: Vec<i32>,
}

impl GameStartDeckCardListResponse {
    pub fn new(draw_card_list: Vec<i32>, deck_card_list: Vec<i32>) -> Self {
        GameStartDeckCardListResponse { draw_card_list, deck_card_list }
    }
}
