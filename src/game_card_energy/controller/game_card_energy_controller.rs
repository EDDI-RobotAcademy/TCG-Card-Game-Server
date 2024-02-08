use async_trait::async_trait;
use crate::game_card_energy::controller::request_form::attach_energy_request_form::AttachEnergyRequestForm;
use crate::game_card_energy::controller::response_form::attach_energy_response_form::AttachEnergyResponseForm;

#[async_trait]
pub trait GameCardEnergyController {
    async fn request_to_attach_energy(
        &self, attach_energy_request_form: AttachEnergyRequestForm) -> AttachEnergyResponseForm;
}