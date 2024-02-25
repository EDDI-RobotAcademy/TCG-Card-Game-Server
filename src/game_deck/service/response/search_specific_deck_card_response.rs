use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchSpecificDeckCardResponse {
    found_card_list: Vec<i32>,
}

impl SearchSpecificDeckCardResponse {
    pub fn new(found_card_list: Vec<i32>,) -> Self { SearchSpecificDeckCardResponse { found_card_list } }
    pub fn get_found_card_list(&self) -> &Vec<i32> { &self.found_card_list }
}