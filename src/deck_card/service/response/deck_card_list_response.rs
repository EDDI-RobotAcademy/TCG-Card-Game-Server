use std::collections::HashMap;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeckCardListResponse {
    card_id_list: Vec<HashMap<i32, i32>>
}

impl DeckCardListResponse {
    pub fn new(card_id_list: Vec<HashMap<i32, i32>>) -> Self { DeckCardListResponse { card_id_list } }
    pub fn get_card_id_list(&self) -> &Vec<HashMap<i32, i32>> { &self.card_id_list }
}