use async_trait::async_trait;
use crate::ui_data_generator::service::request::generate_use_energy_card_to_my_specific_unit_data_request::GenerateUseEnergyCardToMySpecificUnitDataRequest;
use crate::ui_data_generator::service::response::generate_use_energy_card_to_my_specific_unit_data_response::GenerateUseEnergyCardToMySpecificUnitDataResponse;

#[async_trait]
pub trait UiDataGeneratorService {
    async fn generate_use_energy_card_to_my_specific_unit_data(
        &mut self, generate_use_energy_card_to_my_specific_unit_data_request: GenerateUseEnergyCardToMySpecificUnitDataRequest)
        -> GenerateUseEnergyCardToMySpecificUnitDataResponse;
}