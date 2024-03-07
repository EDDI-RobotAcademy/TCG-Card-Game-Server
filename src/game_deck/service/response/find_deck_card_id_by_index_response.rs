use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FindDeckCardIdByIndexResponse {
    found_card_id: i32,
}

impl FindDeckCardIdByIndexResponse {
    pub fn new(found_card_id: i32) -> Self { FindDeckCardIdByIndexResponse { found_card_id } }
    pub fn get_found_card_id(&self) -> i32 { self.found_card_id }
}