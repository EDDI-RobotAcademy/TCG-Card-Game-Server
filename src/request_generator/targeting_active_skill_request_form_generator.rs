use serde_json::Value as JsonValue;
use crate::game_card_active_skill::controller::request_form::targeting_active_skill_request_form::TargetingActiveSkillRequestForm;

pub fn create_targeting_active_skill_request_form(data: &JsonValue) -> Option<TargetingActiveSkillRequestForm> {
    if let (Some(sessionInfo), Some(unit_card_index), Some(opponent_target_card_index), Some(usage_skill_index)) = (
        data.get("sessionInfo").and_then(|v| v.as_str()),
        data.get("unitCardIndex").and_then(|v| v.as_str()),
        data.get("opponentTargetCardIndex").and_then(|v| v.as_str()),
        data.get("usageSkillIndex").and_then(|v| v.as_str()),
    ) {
        Some(TargetingActiveSkillRequestForm::new(
            sessionInfo.to_string(),
            unit_card_index.to_string(),
            opponent_target_card_index.to_string(),
            usage_skill_index.to_string()))
    } else {
        None
    }
}
