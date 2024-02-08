use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UseGameHandSupportCardResponse {
    found_card_id: i32,
}

impl UseGameHandSupportCardResponse {
    pub fn new(found_card_id: i32) -> Self {
        UseGameHandSupportCardResponse { found_card_id }
    }

    pub fn found_card_id(&self) -> i32 {
        self.found_card_id
    }
}
