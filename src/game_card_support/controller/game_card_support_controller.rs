use async_trait::async_trait;
use crate::game_card_support::controller::request_form::draw_support_request_form::DrawSupportRequestForm;
use crate::game_card_support::controller::request_form::energy_boost_support_request_form::EnergyBoostSupportRequestForm;
use crate::game_card_support::controller::request_form::search_unit_support_request_form::SearchUnitSupportRequestForm;
use crate::game_card_support::controller::response_form::draw_support_response_form::DrawSupportResponseForm;
use crate::game_card_support::controller::response_form::energy_boost_support_response_form::EnergyBoostSupportResponseForm;
use crate::game_card_support::controller::response_form::search_unit_support_response_form::SearchUnitSupportResponseForm;

#[async_trait]
pub trait GameCardSupportController {
    async fn request_to_use_energy_boost_support(&self, energy_boost_support_request_form: EnergyBoostSupportRequestForm) -> EnergyBoostSupportResponseForm;
    async fn request_to_use_draw_support(&self, draw_support_request_form: DrawSupportRequestForm) -> DrawSupportResponseForm;
    async fn request_to_use_search_unit_support(&self, search_unit_support_request_form: SearchUnitSupportRequestForm) -> SearchUnitSupportResponseForm;
}