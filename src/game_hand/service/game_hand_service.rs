use async_trait::async_trait;
use crate::game_hand::service::request::use_game_hand_energy_card_request::UseGameHandEnergyCardRequest;
use crate::game_hand::service::request::use_game_hand_unit_card_request::UseGameHandUnitCardRequest;
use crate::game_hand::service::response::use_game_hand_energy_card_response::UseGameHandEnergyCardResponse;
use crate::game_hand::service::response::use_game_hand_unit_card_response::UseGameHandUnitCardResponse;

#[async_trait]
pub trait GameHandService {
    async fn use_specific_card(&mut self, use_game_hand_unit_card_request: UseGameHandUnitCardRequest) -> UseGameHandUnitCardResponse;
    async fn attach_energy_card_to_field_unit(&mut self, use_game_hand_energy_card_request: UseGameHandEnergyCardRequest) -> UseGameHandEnergyCardResponse;
}