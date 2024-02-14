use async_trait::async_trait;

use crate::game_hand::service::request::put_cards_on_deck_request::PutCardsOnDeckRequest;
use crate::game_hand::service::request::use_game_hand_energy_card_request::UseGameHandEnergyCardRequest;
use crate::game_hand::service::request::use_game_hand_item_card_request::UseGameHandItemCardRequest;
use crate::game_hand::service::request::use_game_hand_support_card_request::UseGameHandSupportCardRequest;
use crate::game_hand::service::request::use_game_hand_unit_card_request::UseGameHandUnitCardRequest;
use crate::game_hand::service::request::use_game_hand_tool_card_request::UseGameHandToolCardRequest;

use crate::game_hand::service::response::put_cards_on_deck_response::PutCardsOnDeckResponse;
use crate::game_hand::service::response::use_game_hand_energy_card_response::UseGameHandEnergyCardResponse;
use crate::game_hand::service::response::use_game_hand_item_card_response::UseGameHandItemCardResponse;
use crate::game_hand::service::response::use_game_hand_support_card_response::UseGameHandSupportCardResponse;
use crate::game_hand::service::response::use_game_hand_unit_card_response::UseGameHandUnitCardResponse;
use crate::game_hand::service::response::use_game_hand_tool_card_response::UseGameHandToolCardResponse;

#[async_trait]
pub trait GameHandService {
    async fn put_hand_cards_to_deck(&mut self, request: PutCardsOnDeckRequest) -> PutCardsOnDeckResponse;
    async fn use_support_card(&mut self, use_game_hand_support_card_request: UseGameHandSupportCardRequest) -> UseGameHandSupportCardResponse;
    async fn use_unit_card(&mut self, use_game_hand_unit_card_request: UseGameHandUnitCardRequest) -> UseGameHandUnitCardResponse;
    async fn use_energy_card(&mut self, use_game_hand_energy_card_request: UseGameHandEnergyCardRequest) -> UseGameHandEnergyCardResponse;
    async fn use_item_card(&mut self, use_game_hand_item_card_request: UseGameHandItemCardRequest) -> UseGameHandItemCardResponse;
    async fn use_tool_card(&mut self, use_game_hand_tool_card_request: UseGameHandToolCardRequest) -> UseGameHandToolCardResponse;
}
