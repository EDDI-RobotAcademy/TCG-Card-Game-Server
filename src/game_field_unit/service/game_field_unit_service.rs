use async_trait::async_trait;
use crate::game_field_unit::service::request::add_unit_to_game_field_request::AddUnitToGameFieldRequest;
use crate::game_field_unit::service::request::attach_multiple_energy_to_field_unit_request::AttachMultipleEnergyToFieldUnitRequest;
use crate::game_field_unit::service::response::add_unit_to_game_field_response::AddUnitToGameFieldResponse;
use crate::game_field_unit::service::response::attach_multiple_energy_to_field_unit_response::AttachMultipleEnergyToFieldUnitResponse;

#[async_trait]
pub trait GameFieldUnitService {
    // fn attach_multiple_energy_to_game_field_unit(&mut self, account_unique_id: i32, unit_card_number: i32, race_number: i32, quantity: i32);
    // fn attach_multiple_energy_to_game_field_unit(&mut self, attach_energy_to_unit_request: AttachEnergyToUnitRequest) -> AttachEnergyToUnitResponse;
    async fn attach_multiple_energy_to_game_field_unit(&mut self, attach_multiple_energy_to_field_unit_request: AttachMultipleEnergyToFieldUnitRequest) -> AttachMultipleEnergyToFieldUnitResponse;
    async fn add_unit_to_game_field(&mut self, add_unit_to_game_field_request: AddUnitToGameFieldRequest) -> AddUnitToGameFieldResponse;
}