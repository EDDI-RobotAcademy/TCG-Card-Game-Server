use async_trait::async_trait;

use crate::game_hand::controller::request_form::use_game_hand_energy_card_request_form::UseGameHandEnergyCardRequestForm;
use crate::game_hand::controller::request_form::use_game_hand_unit_card_request_form::UseGameHandUnitCardRequestForm;
use crate::game_hand::controller::response_form::use_game_hand_energy_card_response_form::UseGameHandEnergyCardResponseForm;
use crate::game_hand::controller::response_form::use_game_hand_unit_card_response_form::UseGameHandUnitCardResponseForm;

#[async_trait]
pub trait  GameHandController {
    async fn use_game_hand_energy_card(
        &self, use_game_hand_energy_card_request_form: UseGameHandEnergyCardRequestForm) -> UseGameHandEnergyCardResponseForm;
    async fn use_game_hand_unit_card(
        &self, use_game_hand_unit_card_request_form: UseGameHandUnitCardRequestForm) -> UseGameHandUnitCardResponseForm;
}