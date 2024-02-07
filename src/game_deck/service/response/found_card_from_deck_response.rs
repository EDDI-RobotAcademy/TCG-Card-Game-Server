use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FoundCardFromDeckResponse {
    found_card_list: Vec<i32>,
}

impl FoundCardFromDeckResponse {
    pub fn new(found_card_list: Vec<i32>) -> Self {
        FoundCardFromDeckResponse { found_card_list }
    }

    pub fn found_card_list(&self) -> &Vec<i32> {
        &self.found_card_list
    }
}
