use serde::{Deserialize, Serialize};
use crate::game_hand::controller::response_form::use_game_hand_unit_card_response_form::UseGameHandUnitCardResponseForm;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LegacyUseGameHandUnitCardResponse {
    is_success: bool,
}

impl LegacyUseGameHandUnitCardResponse {
    pub fn new(is_success: bool) -> Self {
        LegacyUseGameHandUnitCardResponse { is_success }
    }
    pub fn to_use_game_hand_unit_card_response_form(&self) -> UseGameHandUnitCardResponseForm {
        UseGameHandUnitCardResponseForm::new( self.is_success )
    }
}
