use async_trait::async_trait;

use crate::game_hand::controller::request_form::mulligan_request_form::MulliganRequestForm;
use crate::game_hand::controller::request_form::use_game_hand_energy_card_request_form::UseGameHandEnergyCardRequestForm;
use crate::game_hand::controller::request_form::use_game_hand_unit_card_request_form::UseGameHandUnitCardRequestForm;
use crate::game_hand::controller::response_form::mulligan_response_form::MulliganResponseForm;
use crate::game_hand::controller::response_form::use_game_hand_energy_card_response_form::UseGameHandEnergyCardResponseForm;
use crate::game_hand::controller::response_form::use_game_hand_unit_card_response_form::UseGameHandUnitCardResponseForm;

#[async_trait]
pub trait  GameHandController {
    async fn execute_mulligan_procedure(
        &self, change_first_hand_request_form: MulliganRequestForm) -> MulliganResponseForm;
}
