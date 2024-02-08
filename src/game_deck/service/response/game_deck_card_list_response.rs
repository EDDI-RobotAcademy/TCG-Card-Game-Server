use serde::{Deserialize, Serialize};

pub struct GameDeckCardListResponse {
    deck_card_list: Vec<i32>
}

impl GameDeckCardListResponse {
    pub fn new(deck_card_list: Vec<i32>) -> Self { GameDeckCardListResponse { deck_card_list } }
    pub fn get_deck_card_list(&self) -> &Vec<i32> { &self.deck_card_list }
}