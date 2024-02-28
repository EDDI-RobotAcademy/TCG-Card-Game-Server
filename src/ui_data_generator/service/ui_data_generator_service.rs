use async_trait::async_trait;
use crate::ui_data_generator::service::request::generate_use_energy_card_to_my_specific_unit_data_request::GenerateUseEnergyCardToMySpecificUnitDataRequest;
use crate::ui_data_generator::service::request::generate_use_field_energy_to_my_specific_unit_data_request::GenerateUseFieldEnergyToMySpecificUnitDataRequest;
use crate::ui_data_generator::service::request::generate_use_support_card_to_boost_energy_to_my_specific_unit_data_request::GenerateUseSupportCardToBoostEnergyToMySpecificUnitDataRequest;
use crate::ui_data_generator::service::request::generate_use_support_card_to_draw_my_deck_data_request::GenerateUseSupportCardToDrawMyDeckDataRequest;
use crate::ui_data_generator::service::request::generate_use_support_card_to_search_unit_from_my_deck_data_request::GenerateUseSupportCardToSearchUnitFromMyDeckDataRequest;
use crate::ui_data_generator::service::response::generate_use_energy_card_to_my_specific_unit_data_response::GenerateUseEnergyCardToMySpecificUnitDataResponse;
use crate::ui_data_generator::service::response::generate_use_field_energy_to_my_specific_unit_data_response::GenerateUseFieldEnergyToMySpecificUnitDataResponse;
use crate::ui_data_generator::service::response::generate_use_support_card_to_boost_energy_to_my_specific_unit_data_response::GenerateUseSupportCardToBoostEnergyToMySpecificUnitDataResponse;
use crate::ui_data_generator::service::response::generate_use_support_card_to_draw_my_deck_data_response::GenerateUseSupportCardToDrawMyDeckDataResponse;
use crate::ui_data_generator::service::response::generate_use_support_card_to_search_unit_from_my_deck_data_response::GenerateUseSupportCardToSearchUnitFromMyDeckDataResponse;

#[async_trait]
pub trait UiDataGeneratorService {
    async fn generate_use_energy_card_to_my_specific_unit_data(
        &mut self,
        generate_use_energy_card_to_my_specific_unit_data_request: GenerateUseEnergyCardToMySpecificUnitDataRequest)
        -> GenerateUseEnergyCardToMySpecificUnitDataResponse;
    async fn generate_use_field_energy_to_my_specific_unit_data(
        &mut self,
        generate_use_field_energy_to_my_specific_unit_data_request: GenerateUseFieldEnergyToMySpecificUnitDataRequest)
        -> GenerateUseFieldEnergyToMySpecificUnitDataResponse;
    async fn generate_use_support_card_to_boost_energy_to_my_specific_unit_data(
        &mut self,
        generate_use_support_card_to_boost_energy_to_my_specific_unit_data_request: GenerateUseSupportCardToBoostEnergyToMySpecificUnitDataRequest)
        -> GenerateUseSupportCardToBoostEnergyToMySpecificUnitDataResponse;
    async fn generate_use_support_card_to_draw_my_deck_data(
        &mut self,
        generate_use_support_card_to_draw_my_deck_data_request: GenerateUseSupportCardToDrawMyDeckDataRequest)
        -> GenerateUseSupportCardToDrawMyDeckDataResponse;
    async fn generate_use_support_card_to_search_unit_from_my_deck_data(
        &mut self,
        generate_use_support_card_to_search_unit_from_my_deck_data_request: GenerateUseSupportCardToSearchUnitFromMyDeckDataRequest)
        -> GenerateUseSupportCardToSearchUnitFromMyDeckDataResponse;
}