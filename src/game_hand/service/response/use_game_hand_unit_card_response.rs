use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UseGameHandUnitCardResponse {
    found_unit_card_id: i32,
}

impl UseGameHandUnitCardResponse {
    pub fn new(found_unit_card_id: i32) -> Self {
        UseGameHandUnitCardResponse { found_unit_card_id }
    }

    pub fn get_found_unit_card_id(&self) -> i32 {
        self.found_unit_card_id
    }
}
