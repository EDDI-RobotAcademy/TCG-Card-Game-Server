use async_trait::async_trait;
use crate::game_card_energy::controller::request_form::attach_general_energy_card_request_form::AttachGeneralEnergyCardRequestForm;
use crate::game_card_energy::controller::request_form::attach_special_energy_card_request_form::AttachSpecialEnergyCardRequestForm;
use crate::game_card_energy::controller::response_form::attach_general_energy_card_response_form::AttachGeneralEnergyCardResponseForm;
use crate::game_card_energy::controller::response_form::attach_special_energy_card_response_form::AttachSpecialEnergyCardResponseForm;

#[async_trait]
pub trait GameCardEnergyController {
    async fn request_to_attach_general_energy(
        &self, attach_energy_request_form: AttachGeneralEnergyCardRequestForm) -> AttachGeneralEnergyCardResponseForm;

    async fn request_to_attach_special_energy(
        &self, attach_special_energy_request_form: AttachSpecialEnergyCardRequestForm) -> AttachSpecialEnergyCardResponseForm;
}