use serde_json::Value as JsonValue;
use crate::game_card_passive_skill::controller::request_form::deploy_targeting_attack_passive_skill_request_form::DeployTargetingAttackPassiveSkillRequestForm;

pub fn create_deploy_targeting_attack_passive_skill_request_form(data: &JsonValue) -> Option<DeployTargetingAttackPassiveSkillRequestForm> {
    if let (Some(sessionInfo), Some(unit_card_index), Some(opponent_target_card_index), Some(usage_skill_index)) = (
        data.get("sessionInfo").and_then(|v| v.as_str()),
        data.get("unitCardIndex").and_then(|v| v.as_str()),
        data.get("opponentTargetCardIndex").and_then(|v| v.as_str()),
        data.get("usageSkillIndex").and_then(|v| v.as_str()),
    ) {
        Some(DeployTargetingAttackPassiveSkillRequestForm::new(
            sessionInfo.to_string(),
            unit_card_index.to_string(),
            opponent_target_card_index.to_string(),
            usage_skill_index.to_string()))
    } else {
        None
    }
}
