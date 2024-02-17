use async_trait::async_trait;
use crate::game_field_energy::service::request::add_field_energy_with_amount_request::AddFieldEnergyWithAmountRequest;
use crate::game_field_energy::service::request::check_field_energy_enough_to_use_request::CheckFieldEnergyEnoughToUseRequest;
use crate::game_field_energy::service::request::remove_field_energy_with_amount_request::RemoveFieldEnergyWithAmountRequest;
use crate::game_field_energy::service::response::add_field_energy_with_amount_response::AddFieldEnergyWithAmountResponse;
use crate::game_field_energy::service::response::check_field_energy_enough_to_use_response::CheckFieldEnergyEnoughToUseResponse;
use crate::game_field_energy::service::response::remove_field_energy_with_amount_response::RemoveFieldEnergyWithAmountResponse;

#[async_trait]
pub trait GameFieldEnergyService {
    async fn add_field_energy_with_amount(&self, add_field_energy_with_amount_request: AddFieldEnergyWithAmountRequest) -> AddFieldEnergyWithAmountResponse;
    async fn remove_field_energy_with_amount(&self, remove_field_energy_with_amount_request: RemoveFieldEnergyWithAmountRequest) -> RemoveFieldEnergyWithAmountResponse;
    async fn check_field_energy_enough_to_use(&self, check_field_energy_enough_to_use_request: CheckFieldEnergyEnoughToUseRequest) -> CheckFieldEnergyEnoughToUseResponse;
}