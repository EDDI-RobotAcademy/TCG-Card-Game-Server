use serde_json::Value as JsonValue;
use crate::game_card_passive_skill::controller::request_form::deploy_targeting_attack_to_game_main_character_request_form::DeployTargetingAttackToGameMainCharacterRequestForm;

pub fn create_deploy_targeting_attack_to_game_main_character_request_form(data: &JsonValue) -> Option<DeployTargetingAttackToGameMainCharacterRequestForm> {
    if let (Some(sessionInfo), Some(unit_card_index), Some(target_game_main_character_index), Some(usage_skill_index)) = (
        data.get("sessionInfo").and_then(|v| v.as_str()),
        data.get("unitCardIndex").and_then(|v| v.as_str()),
        data.get("targetGameMainCharacterIndex").and_then(|v| v.as_str()),
        data.get("usageSkillIndex").and_then(|v| v.as_str()),
    ) {
        Some(DeployTargetingAttackToGameMainCharacterRequestForm::new(
            sessionInfo.to_string(),
            unit_card_index.to_string(),
            target_game_main_character_index.to_string(),
            usage_skill_index.to_string()))
    } else {
        None
    }
}
