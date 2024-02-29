use async_trait::async_trait;
use crate::ui_data_generator::service::request::generate_my_specific_unit_energy_data_request::GenerateMySpecificUnitEnergyDataRequest;
use crate::ui_data_generator::service::request::generate_my_field_energy_data_request::{GenerateMyFieldEnergyDataRequest};
use crate::ui_data_generator::service::request::generate_opponent_specific_unit_death_data_request::{GenerateOpponentSpecificUnitDeathDataRequest};
use crate::ui_data_generator::service::request::generate_use_my_hand_card_data_request::GenerateUseMyHandCardDataRequest;
use crate::ui_data_generator::service::request::generate_use_my_deck_card_list_data_request::GenerateUseMyDeckCardListDataRequest;
use crate::ui_data_generator::service::request::generate_draw_my_deck_data_request::GenerateDrawMyDeckDataRequest;
use crate::ui_data_generator::service::request::generate_draw_opponent_deck_data_request::GenerateDrawOpponentDeckDataRequest;
use crate::ui_data_generator::service::request::generate_my_multiple_unit_health_point_data_request::GenerateMyMultipleUnitHealthPointDataRequest;
use crate::ui_data_generator::service::request::generate_my_specific_unit_health_point_data_request::GenerateMySpecificUnitHealthPointDataRequest;
use crate::ui_data_generator::service::request::generate_opponent_field_energy_data_request::GenerateOpponentFieldEnergyDataRequest;
use crate::ui_data_generator::service::request::generate_opponent_multiple_unit_health_point_data_request::GenerateOpponentMultipleUnitHealthPointDataRequest;
use crate::ui_data_generator::service::request::generate_opponent_specific_unit_health_point_data_request::GenerateOpponentSpecificUnitHealthPointDataRequest;
use crate::ui_data_generator::service::request::generate_search_my_deck_data_request::GenerateSearchMyDeckDataRequest;
use crate::ui_data_generator::service::response::generate_my_specific_unit_energy_data_response::GenerateMySpecificUnitEnergyDataResponse;
use crate::ui_data_generator::service::response::generate_my_field_energy_data_response::{GenerateMyFieldEnergyDataResponse};
use crate::ui_data_generator::service::response::generate_opponent_specific_unit_death_data_response::{GenerateOpponentSpecificUnitDeathDataResponse};
use crate::ui_data_generator::service::response::generate_use_my_hand_card_data_response::GenerateUseMyHandCardDataResponse;
use crate::ui_data_generator::service::response::generate_use_my_deck_card_list_data_response::GenerateUseMyDeckCardListDataResponse;
use crate::ui_data_generator::service::response::generate_draw_my_deck_data_response::GenerateDrawMyDeckDataResponse;
use crate::ui_data_generator::service::response::generate_draw_opponent_deck_data_response::GenerateDrawOpponentDeckDataResponse;
use crate::ui_data_generator::service::response::generate_my_multiple_unit_health_point_data_response::GenerateMyMultipleUnitHealthPointDataResponse;
use crate::ui_data_generator::service::response::generate_my_specific_unit_health_point_data_response::GenerateMySpecificUnitHealthPointDataResponse;
use crate::ui_data_generator::service::response::generate_opponent_field_energy_data_response::GenerateOpponentFieldEnergyDataResponse;
use crate::ui_data_generator::service::response::generate_opponent_multiple_unit_health_point_data_response::GenerateOpponentMultipleUnitHealthPointDataResponse;
use crate::ui_data_generator::service::response::generate_opponent_specific_unit_health_point_data_response::GenerateOpponentSpecificUnitHealthPointDataResponse;
use crate::ui_data_generator::service::response::generate_search_my_deck_data_response::GenerateSearchMyDeckDataResponse;

#[async_trait]
pub trait UiDataGeneratorService {
    async fn generate_use_my_hand_card_data(
        &mut self,
        generate_use_my_hand_card_data_request: GenerateUseMyHandCardDataRequest)
        -> GenerateUseMyHandCardDataResponse;
    async fn generate_my_specific_unit_health_point_data(
        &mut self,
        generate_my_specific_unit_health_point_data_request: GenerateMySpecificUnitHealthPointDataRequest)
        -> GenerateMySpecificUnitHealthPointDataResponse;
    async fn generate_my_multiple_unit_health_point_data(
        &mut self, generate_opponent_multiple_unit_health_point_data_request: GenerateMyMultipleUnitHealthPointDataRequest)
        -> GenerateMyMultipleUnitHealthPointDataResponse;
    async fn generate_my_specific_unit_energy_data(
        &mut self,
        generate_my_specific_unit_energy_data_request: GenerateMySpecificUnitEnergyDataRequest)
        -> GenerateMySpecificUnitEnergyDataResponse;
    async fn generate_my_field_energy_data(
        &mut self,
        generate_my_field_energy_data_request: GenerateMyFieldEnergyDataRequest)
        -> GenerateMyFieldEnergyDataResponse;
    async fn generate_use_my_deck_card_list_data(
        &mut self,
        generate_use_my_deck_card_list_data_request: GenerateUseMyDeckCardListDataRequest)
        -> GenerateUseMyDeckCardListDataResponse;
    async fn generate_draw_my_deck_data(
        &mut self,
        generate_draw_my_deck_data_request: GenerateDrawMyDeckDataRequest)
        -> GenerateDrawMyDeckDataResponse;
    async fn generate_draw_opponent_deck_data(
        &mut self,
        generate_draw_opponent_deck_data_request: GenerateDrawOpponentDeckDataRequest)
        -> GenerateDrawOpponentDeckDataResponse;
    async fn generate_search_my_deck_data(
        &mut self,
        generate_search_my_deck_data_request: GenerateSearchMyDeckDataRequest)
        -> GenerateSearchMyDeckDataResponse;
    async fn generate_opponent_field_energy_data(
        &mut self,
        generate_opponent_field_energy_data_request: GenerateOpponentFieldEnergyDataRequest)
        -> GenerateOpponentFieldEnergyDataResponse;
    async fn generate_opponent_specific_unit_death_data(
        &mut self,
        generate_opponent_specific_unit_death_data_request: GenerateOpponentSpecificUnitDeathDataRequest)
        -> GenerateOpponentSpecificUnitDeathDataResponse;
    async fn generate_opponent_specific_unit_health_point_data(
        &mut self,
        generate_opponent_specific_unit_health_point_data_request: GenerateOpponentSpecificUnitHealthPointDataRequest)
        -> GenerateOpponentSpecificUnitHealthPointDataResponse;
    async fn generate_opponent_multiple_unit_health_point_data(
        &mut self, generate_opponent_multiple_unit_health_point_data_request: GenerateOpponentMultipleUnitHealthPointDataRequest)
        -> GenerateOpponentMultipleUnitHealthPointDataResponse;
}