use async_trait::async_trait;
use crate::ui_data_generator::service::request::generate_my_specific_unit_energy_data_request::GenerateMySpecificUnitEnergyDataRequest;
use crate::ui_data_generator::service::request::generate_use_my_field_energy_data_request::{GenerateUseMyFieldEnergyDataRequest};
use crate::ui_data_generator::service::request::generate_instant_death_of_your_specific_unit_data_request::{GenerateInstantDeathOfYourSpecificUnitDataRequest};
use crate::ui_data_generator::service::request::generate_use_my_hand_card_data_request::GenerateUseMyHandCardDataRequest;
use crate::ui_data_generator::service::request::generate_use_support_card_to_boost_energy_to_my_specific_unit_data_request::GenerateUseSupportCardToBoostEnergyToMySpecificUnitDataRequest;
use crate::ui_data_generator::service::request::generate_use_support_card_to_draw_my_deck_data_request::GenerateUseSupportCardToDrawMyDeckDataRequest;
use crate::ui_data_generator::service::request::generate_use_support_card_to_remove_your_field_energy_data_request::GenerateUseSupportCardToRemoveYourFieldEnergyDataRequest;
use crate::ui_data_generator::service::request::generate_use_support_card_to_search_unit_from_my_deck_data_request::GenerateUseSupportCardToSearchUnitFromMyDeckDataRequest;
use crate::ui_data_generator::service::response::generate_my_specific_unit_energy_data_response::GenerateMySpecificUnitEnergyDataResponse;
use crate::ui_data_generator::service::response::generate_use_my_field_energy_data_response::{GenerateUseMyFieldEnergyDataResponse};
use crate::ui_data_generator::service::response::generate_instant_death_of_your_specific_unit_data_response::{GenerateInstantDeathOfYourSpecificUnitDataResponse};
use crate::ui_data_generator::service::response::generate_use_my_hand_card_data_response::GenerateUseMyHandCardDataResponse;
use crate::ui_data_generator::service::response::generate_use_support_card_to_boost_energy_to_my_specific_unit_data_response::GenerateUseSupportCardToBoostEnergyToMySpecificUnitDataResponse;
use crate::ui_data_generator::service::response::generate_use_support_card_to_draw_my_deck_data_response::GenerateUseSupportCardToDrawMyDeckDataResponse;
use crate::ui_data_generator::service::response::generate_use_support_card_to_remove_your_field_energy_data_response::GenerateUseSupportCardToRemoveYourFieldEnergyDataResponse;
use crate::ui_data_generator::service::response::generate_use_support_card_to_search_unit_from_my_deck_data_response::GenerateUseSupportCardToSearchUnitFromMyDeckDataResponse;

#[async_trait]
pub trait UiDataGeneratorService {
    async fn generate_use_my_hand_card_data(
        &mut self,
        generate_use_my_hand_card_data_request: GenerateUseMyHandCardDataRequest)
        -> GenerateUseMyHandCardDataResponse;
    async fn generate_my_specific_unit_energy_data(
        &mut self,
        generate_my_specific_unit_energy_data_request: GenerateMySpecificUnitEnergyDataRequest)
        -> GenerateMySpecificUnitEnergyDataResponse;
    async fn generate_use_my_field_energy_data(
        &mut self,
        generate_use_my_field_energy_data_request: GenerateUseMyFieldEnergyDataRequest)
        -> GenerateUseMyFieldEnergyDataResponse;
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
    async fn generate_use_support_card_to_remove_your_field_energy_data(
        &mut self,
        generate_use_support_card_to_remove_your_field_energy_data_request: GenerateUseSupportCardToRemoveYourFieldEnergyDataRequest)
        -> GenerateUseSupportCardToRemoveYourFieldEnergyDataResponse;
    async fn generate_instant_death_of_your_specific_unit_data(
        &mut self,
        generate_instant_death_of_your_specific_unit_data_request: GenerateInstantDeathOfYourSpecificUnitDataRequest)
        -> GenerateInstantDeathOfYourSpecificUnitDataResponse;
}