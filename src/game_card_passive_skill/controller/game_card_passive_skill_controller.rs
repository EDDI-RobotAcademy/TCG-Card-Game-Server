use async_trait::async_trait;
use crate::game_card_passive_skill::controller::request_form::non_targeting_passive_skill_request_form::NonTargetingPassiveSkillRequestForm;
use crate::game_card_passive_skill::controller::request_form::targeting_passive_skill_request_form::TargetingPassiveSkillRequestForm;
use crate::game_card_passive_skill::controller::response_form::non_targeting_passive_skill_response_form::NonTargetingPassiveSkillResponseForm;
use crate::game_card_passive_skill::controller::response_form::targeting_passive_skill_response_form::TargetingPassiveSkillResponseForm;

#[async_trait]
pub trait GameCardPassiveSkillController {
    async fn request_targeting_active_skill(
        &self, targeting_passive_skill_request_form: TargetingPassiveSkillRequestForm)
        -> TargetingPassiveSkillResponseForm;

    async fn request_non_targeting_active_skill(
        &self, non_targeting_passive_skill_request_form: NonTargetingPassiveSkillRequestForm)
        -> NonTargetingPassiveSkillResponseForm;
}
