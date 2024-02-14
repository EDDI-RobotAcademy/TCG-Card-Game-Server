use async_trait::async_trait;
use crate::game_protocol_validation::service::request::can_use_card_request::CanUseCardRequest;
use crate::game_protocol_validation::service::request::check_cards_from_hand_request::CheckCardsFromHandRequest;
use crate::game_protocol_validation::service::request::check_protocol_hacking_request::CheckProtocolHackingRequest;
use crate::game_protocol_validation::service::request::is_it_energy_card_request::IsItEnergyCardRequest;
use crate::game_protocol_validation::service::request::is_it_item_card_request::IsItItemCardRequest;
use crate::game_protocol_validation::service::request::is_it_support_card_request::IsItSupportCardRequest;
use crate::game_protocol_validation::service::request::is_it_unit_card_request::IsItUnitCardRequest;
use crate::game_protocol_validation::service::request::is_it_tool_card_request::IsItToolCardRequest;
use crate::game_protocol_validation::service::request::support_card_protocol_validation_request::SupportCardProtocolValidationRequest;

use crate::game_protocol_validation::service::response::can_use_card_response::CanUseCardResponse;
use crate::game_protocol_validation::service::response::check_cards_from_hand_response::CheckCardsFromHandResponse;
use crate::game_protocol_validation::service::response::check_protocol_hacking_response::CheckProtocolHackingResponse;
use crate::game_protocol_validation::service::response::is_it_energy_card_response::IsItEnergyCardResponse;
use crate::game_protocol_validation::service::response::is_it_item_card_response::IsItItemCardResponse;
use crate::game_protocol_validation::service::response::is_it_support_card_response::IsItSupportCardResponse;
use crate::game_protocol_validation::service::response::is_it_unit_card_response::IsItUnitCardResponse;
use crate::game_protocol_validation::service::response::is_it_tool_card_response::IsItToolCardResponse;
use crate::game_protocol_validation::service::response::support_card_protocol_validation_response::SupportCardProtocolValidationResponse;

#[async_trait]
pub trait GameProtocolValidationService {
    async fn check_protocol_hacking(&mut self, check_protocol_hacking_request: CheckProtocolHackingRequest) -> CheckProtocolHackingResponse;
    async fn check_cards_from_hand(&mut self, check_cards_from_hand_request: CheckCardsFromHandRequest) -> CheckCardsFromHandResponse;
    async fn can_use_card(&mut self, can_use_card_request: CanUseCardRequest) -> CanUseCardResponse;
    async fn is_it_support_card(&self, is_it_support_card_request: IsItSupportCardRequest) -> IsItSupportCardResponse;
    async fn is_it_unit_card(&self, is_it_unit_card_request: IsItUnitCardRequest) -> IsItUnitCardResponse;
    async fn is_it_energy_card(&self, is_it_energy_card_request: IsItEnergyCardRequest) -> IsItEnergyCardResponse;
    async fn is_it_item_card(&self, is_it_item_card_request: IsItItemCardRequest) -> IsItItemCardResponse;
    async fn is_it_tool_card(&self, is_it_tool_card_request: IsItToolCardRequest) -> IsItToolCardResponse;
}
