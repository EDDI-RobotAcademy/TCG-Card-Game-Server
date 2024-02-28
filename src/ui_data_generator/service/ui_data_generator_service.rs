use async_trait::async_trait;
use crate::ui_data_generator::service::request::generate_use_energy_card_to_my_specific_unit_data_request::GenerateUseEnergyCardToMySpecificUnitDataRequest;
use crate::ui_data_generator::service::request::generate_use_field_energy_to_my_specific_unit_request::GenerateUseFieldEnergyToMySpecificUnitRequest;
use crate::ui_data_generator::service::request::generate_use_support_card_to_boost_energy_to_my_specific_unit_request::GenerateUseSupportCardToBoostEnergyToMySpecificUnitRequest;
use crate::ui_data_generator::service::response::generate_use_energy_card_to_my_specific_unit_data_response::GenerateUseEnergyCardToMySpecificUnitDataResponse;
use crate::ui_data_generator::service::response::generate_use_field_energy_to_my_specific_unit_response::GenerateUseFieldEnergyToMySpecificUnitResponse;
use crate::ui_data_generator::service::response::generate_use_support_card_to_boost_energy_to_my_specific_unit_response::GenerateUseSupportCardToBoostEnergyToMySpecificUnitResponse;

#[async_trait]
pub trait UiDataGeneratorService {
    async fn generate_use_energy_card_to_my_specific_unit_data(
        &mut self,
        generate_use_energy_card_to_my_specific_unit_data_request: GenerateUseEnergyCardToMySpecificUnitDataRequest)
        -> GenerateUseEnergyCardToMySpecificUnitDataResponse;
    async fn generate_use_field_energy_to_my_specific_unit_data(
        &mut self,
        generate_use_field_energy_to_my_specific_unit_request: GenerateUseFieldEnergyToMySpecificUnitRequest)
        -> GenerateUseFieldEnergyToMySpecificUnitResponse;
    async fn generate_use_support_card_to_boost_energy_to_my_specific_unit(
        &mut self,
        generate_use_support_card_to_boost_energy_to_my_specific_unit_request: GenerateUseSupportCardToBoostEnergyToMySpecificUnitRequest)
        -> GenerateUseSupportCardToBoostEnergyToMySpecificUnitResponse;
}