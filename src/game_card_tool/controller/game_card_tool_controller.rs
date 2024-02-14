use async_trait::async_trait;
use crate::game_card_tool::controller::request_form::enhance_attack_point_tool_request_form::EnhanceAttackPointToolRequestForm;
use crate::game_card_tool::controller::response_form::enhance_attack_point_tool_response_form::EnhanceAttackPointToolResponseForm;

#[async_trait]
pub trait GameCardToolController {
    async fn request_to_use_enhance_attack_point_tool(
        &self, enhance_attack_point_tool_request_form: EnhanceAttackPointToolRequestForm) -> EnhanceAttackPointToolResponseForm;
}