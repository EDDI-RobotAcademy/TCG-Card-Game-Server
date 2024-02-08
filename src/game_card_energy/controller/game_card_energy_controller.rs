use async_trait::async_trait;
use crate::game_card_energy::controller::request_form::attach_general_energy_card_request_form::AttachGeneralEnergyCardRequestForm;
use crate::game_card_energy::controller::response_form::attach_general_energy_card_response_form::AttachGeneralEnergyCardResponseForm;

#[async_trait]
pub trait GameCardEnergyController {
    async fn request_to_attach_general_energy(
        &self, attach_energy_request_form: AttachGeneralEnergyCardRequestForm) -> AttachGeneralEnergyCardResponseForm;
}