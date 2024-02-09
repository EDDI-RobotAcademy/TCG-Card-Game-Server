use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UseGameHandItemCardResponse {
    found_item_card_id: i32,
}

impl UseGameHandItemCardResponse {
    pub fn new(found_item_card_id: i32) -> Self {
        UseGameHandItemCardResponse { found_item_card_id }
    }

    pub fn get_found_item_card_id(&self) -> i32 {
        self.found_item_card_id
    }
}
