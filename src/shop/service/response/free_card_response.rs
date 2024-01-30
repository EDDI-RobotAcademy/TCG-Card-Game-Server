use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FreeCardResponse {
    card_id_list: Vec<i32>
}

impl FreeCardResponse {
    pub fn new(card_id_list: Vec<i32>) -> Self {
        FreeCardResponse { card_id_list }
    }
    pub fn get_card_id_list(&self) -> &Vec<i32> { &self.card_id_list }
}