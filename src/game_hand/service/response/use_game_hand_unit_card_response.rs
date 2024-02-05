use serde::{Deserialize, Serialize};
use crate::game_hand::controller::response_form::use_game_hand_unit_card_response_form::UseGameHandUnitCardResponseForm;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UseGameHandUnitCardResponse {
    is_success: bool,
}

impl UseGameHandUnitCardResponse {
    pub fn new(is_success: bool) -> Self {
        UseGameHandUnitCardResponse { is_success }
    }
    pub fn to_use_game_hand_unit_card_response_form(&self) -> UseGameHandUnitCardResponseForm {
        UseGameHandUnitCardResponseForm::new( self.is_success )
    }
}
