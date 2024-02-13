use async_trait::async_trait;
use crate::game_card_active_skill::controller::request_form::targeting_active_skill_request_form::TargetingActiveSkillRequestForm;
use crate::game_card_active_skill::controller::response_form::targeting_active_skill_response_form::TargetingActiveSkillResponseForm;

#[async_trait]
pub trait GameCardActiveSkillController {
    async fn request_targeting_active_skill(
        &self, targeting_active_skill_request_form: TargetingActiveSkillRequestForm) -> TargetingActiveSkillResponseForm;
}
