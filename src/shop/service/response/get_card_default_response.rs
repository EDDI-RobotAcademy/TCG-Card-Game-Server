use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetCardDefaultResponse {
    card_id_list: Vec<i32>
}

impl GetCardDefaultResponse {
    pub fn new(card_id_list: Vec<i32>) -> Self {
        GetCardDefaultResponse { card_id_list }
    }
    pub fn get_card_id_list(&self) -> &Vec<i32> { &self.card_id_list }
}