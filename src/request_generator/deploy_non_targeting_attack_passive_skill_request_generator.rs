use serde_json::Value as JsonValue;
use crate::game_card_passive_skill::controller::request_form::deploy_non_targeting_attack_passive_skill_request_form::DeployNonTargetingAttackPassiveSkillRequestForm;

pub fn create_deploy_non_targeting_attack_passive_skill_request_form(data: &JsonValue) -> Option<DeployNonTargetingAttackPassiveSkillRequestForm> {
    if let (Some(sessionInfo), Some(unit_card_index), Some(usage_skill_index)) = (
        data.get("sessionInfo").and_then(|v| v.as_str()),
        data.get("unitCardIndex").and_then(|v| v.as_str()),
        data.get("usageSkillIndex").and_then(|v| v.as_str()),
    ) {
        Some(DeployNonTargetingAttackPassiveSkillRequestForm::new(
            sessionInfo.to_string(),
            unit_card_index.to_string(),
            usage_skill_index.to_string()))
    } else {
        None
    }
}