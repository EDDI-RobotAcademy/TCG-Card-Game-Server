use serde::{Deserialize, Serialize};
use crate::game_hand::controller::response_form::use_game_hand_unit_card_response_form::UseGameHandUnitCardResponseForm;

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
