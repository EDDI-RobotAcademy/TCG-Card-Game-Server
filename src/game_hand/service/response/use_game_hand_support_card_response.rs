use serde::{Deserialize, Serialize};
use crate::game_hand::controller::response_form::use_game_hand_unit_card_response_form::UseGameHandUnitCardResponseForm;

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
