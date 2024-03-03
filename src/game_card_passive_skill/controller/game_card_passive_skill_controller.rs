use async_trait::async_trait;
use crate::game_card_passive_skill::controller::request_form::deploy_non_targeting_attack_passive_skill_request_form::DeployNonTargetingAttackPassiveSkillRequestForm;
use crate::game_card_passive_skill::controller::request_form::deploy_targeting_attack_passive_skill_request_form::DeployTargetingAttackPassiveSkillRequestForm;
use crate::game_card_passive_skill::controller::response_form::deploy_non_targeting_attack_passive_skill_response_form::DeployNonTargetingAttackPassiveSkillResponseForm;
use crate::game_card_passive_skill::controller::response_form::deploy_targeting_attack_passive_skill_response_form::DeployTargetingAttackPassiveSkillResponseForm;

#[async_trait]
pub trait GameCardPassiveSkillController {
    async fn request_deploy_targeting_attack_passive_skill(
        &self, targeting_passive_skill_request_form: DeployTargetingAttackPassiveSkillRequestForm)
        -> DeployTargetingAttackPassiveSkillResponseForm;

    async fn request_deploy_non_targeting_attack_passive_skill(
        &self, non_targeting_passive_skill_request_form: DeployNonTargetingAttackPassiveSkillRequestForm)
        -> DeployNonTargetingAttackPassiveSkillResponseForm;
}
