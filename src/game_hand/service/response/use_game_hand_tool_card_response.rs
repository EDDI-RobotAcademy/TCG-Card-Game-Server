use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UseGameHandToolCardResponse {
    found_card_id: i32,
}

impl UseGameHandToolCardResponse {
    pub fn new(found_card_id: i32) -> Self {
        UseGameHandToolCardResponse { found_card_id }
    }

    pub fn found_card_id(&self) -> i32 {
        self.found_card_id
    }
}