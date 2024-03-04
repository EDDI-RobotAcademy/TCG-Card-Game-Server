use async_trait::async_trait;
use crate::game_card_passive_skill::controller::request_form::deploy_non_targeting_attack_passive_skill_request_form::DeployNonTargetingAttackPassiveSkillRequestForm;
use crate::game_card_passive_skill::controller::request_form::deploy_targeting_attack_passive_skill_request_form::DeployTargetingAttackPassiveSkillRequestForm;
use crate::game_card_passive_skill::controller::request_form::turn_start_non_targeting_attack_passive_skill_request_form::TurnStartNonTargetingAttackPassiveSkillRequestForm;
use crate::game_card_passive_skill::controller::request_form::turn_start_targeting_attack_passive_skill_request_form::TurnStartTargetingAttackPassiveSkillRequestForm;
use crate::game_card_passive_skill::controller::response_form::deploy_non_targeting_attack_passive_skill_response_form::DeployNonTargetingAttackPassiveSkillResponseForm;
use crate::game_card_passive_skill::controller::response_form::deploy_targeting_attack_passive_skill_response_form::DeployTargetingAttackPassiveSkillResponseForm;
use crate::game_card_passive_skill::controller::response_form::turn_start_non_targeting_attack_passive_skill_response_form::TurnStartNonTargetingAttackPassiveSkillResponseForm;
use crate::game_card_passive_skill::controller::response_form::turn_start_targeting_attack_passive_skill_response_form::TurnStartTargetingAttackPassiveSkillResponseForm;

#[async_trait]
pub trait GameCardPassiveSkillController {
    async fn request_deploy_targeting_attack_passive_skill(
        &self, deploy_targeting_passive_skill_request_form: DeployTargetingAttackPassiveSkillRequestForm)
        -> DeployTargetingAttackPassiveSkillResponseForm;

    async fn request_deploy_non_targeting_attack_passive_skill(
        &self, deploy_non_targeting_passive_skill_request_form: DeployNonTargetingAttackPassiveSkillRequestForm)
        -> DeployNonTargetingAttackPassiveSkillResponseForm;

    async fn request_turn_start_targeting_attack_passive_skill(
        &self, turn_start_targeting_attack_passive_skill_request_form: TurnStartTargetingAttackPassiveSkillRequestForm)
        -> TurnStartTargetingAttackPassiveSkillResponseForm;
    async fn request_turn_start_non_targeting_attack_passive_skill(
        &self, turn_start_non_targeting_attack_passive_skill_request_form: TurnStartNonTargetingAttackPassiveSkillRequestForm)
        -> TurnStartNonTargetingAttackPassiveSkillResponseForm;
}
