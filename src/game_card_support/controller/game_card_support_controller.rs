use async_trait::async_trait;
use crate::game_card_support::controller::request_form::energy_boost_support_request_form::EnergyBoostSupportRequestForm;
use crate::game_card_support::controller::response_form::energy_boost_support_response_form::EnergyBoostSupportResponseForm;

#[async_trait]
pub trait GameCardSupportController {
    async fn request_to_use_energy_boost_support(
        &self, energy_boost_support_request_form: EnergyBoostSupportRequestForm) -> EnergyBoostSupportResponseForm;
}