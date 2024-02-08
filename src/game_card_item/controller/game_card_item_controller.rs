use async_trait::async_trait;
use crate::game_card_item::controller::request_form::target_death_item_request_form::TargetDeathItemRequestForm;
use crate::game_card_item::controller::response_form::target_death_item_response_form::TargetDeathItemResponseForm;

#[async_trait]
pub trait GameCardItemController {
    async fn request_to_use_target_death_item(
        &self, target_death_item_request_form: TargetDeathItemRequestForm) -> TargetDeathItemResponseForm;
}
