use serde_json::Value as JsonValue;
use crate::game_turn::controller::request_form::first_turn_decision_request_form::FirstTurnDecisionRequestForm;

pub fn create_first_turn_decision_request_form(data: &JsonValue) -> Option<FirstTurnDecisionRequestForm> {
    if let (((Some(sessionInfo),))) = (
        data.get("sessionInfo").and_then(|v| v.as_str()),


    ) {
        Some(FirstTurnDecisionRequestForm::new(
            sessionInfo.to_string(), ))
    } else {
        None
    }
}
