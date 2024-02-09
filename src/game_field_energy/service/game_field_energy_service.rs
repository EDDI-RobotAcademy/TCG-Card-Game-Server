use async_trait::async_trait;
use crate::game_field_energy::service::request::remove_field_energy_with_amount_request::RemoveFieldEnergyWithAmountRequest;
use crate::game_field_energy::service::response::remove_field_energy_with_amount_response::RemoveFieldEnergyWithAmountResponse;

#[async_trait]
pub trait GameFieldEnergyService {
    async fn remove_field_energy_with_amount(
        &self, remove_field_energy_with_amount_request: RemoveFieldEnergyWithAmountRequest) -> RemoveFieldEnergyWithAmountResponse;
}