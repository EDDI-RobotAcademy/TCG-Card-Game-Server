use async_trait::async_trait;
use crate::game_field_energy::controller::request_form::attach_field_energy_to_field_unit_request_form::AttachFieldEnergyToFieldUnitRequestForm;
use crate::game_field_energy::controller::response_form::attach_field_energy_to_field_unit_response_form::AttachFieldEnergyToFieldUnitResponseForm;

#[async_trait]
pub trait GameFieldEnergyController {
    async fn request_to_attach_field_energy_to_field_unit(
        &self, attach_field_energy_to_field_unit_request_form: AttachFieldEnergyToFieldUnitRequestForm)
        -> AttachFieldEnergyToFieldUnitResponseForm;
}