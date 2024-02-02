use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameStartDeckCardListResponse {
    hand_card_list: Vec<i32>,
    // deck_card_list: Vec<i32>,
}

impl GameStartDeckCardListResponse {
    // pub fn new(hand_card_list: Vec<i32>, deck_card_list: Vec<i32>) -> Self {
    //     GameStartDeckCardListResponse { hand_card_list, deck_card_list }
    // }
    pub fn new(hand_card_list: Vec<i32>) -> Self {
        GameStartDeckCardListResponse { hand_card_list }
    }
}
