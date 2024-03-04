use serde_json::Value as JsonValue;
use crate::game_card_passive_skill::controller::request_form::turn_start_non_targeting_attack_passive_skill_request_form::TurnStartNonTargetingAttackPassiveSkillRequestForm;

pub fn create_turn_start_non_targeting_attack_passive_skill_request_form(data: &JsonValue) -> Option<TurnStartNonTargetingAttackPassiveSkillRequestForm> {
    if let (Some(sessionInfo), Some(unit_card_index), Some(usage_skill_index)) = (
        data.get("sessionInfo").and_then(|v| v.as_str()),
        data.get("unitCardIndex").and_then(|v| v.as_str()),
        data.get("usageSkillIndex").and_then(|v| v.as_str()),
    ) {
        Some(TurnStartNonTargetingAttackPassiveSkillRequestForm::new(
            sessionInfo.to_string(),
            unit_card_index.to_string(),
            usage_skill_index.to_string()))
    } else {
        None
    }
}