use serde::{Deserialize, Serialize};
use crate::game_hand::controller::response_form::use_game_hand_energy_card_response_form::UseGameHandEnergyCardResponseForm;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LegacyUseGameHandEnergyCardResponse {
    is_success: bool,
}

impl LegacyUseGameHandEnergyCardResponse {
    pub fn new(is_success: bool) -> Self {
        LegacyUseGameHandEnergyCardResponse { is_success }
    }
    pub fn to_use_game_hand_energy_card_response_form(&self) -> UseGameHandEnergyCardResponseForm {
        UseGameHandEnergyCardResponseForm::new( self.is_success )
    }
}
