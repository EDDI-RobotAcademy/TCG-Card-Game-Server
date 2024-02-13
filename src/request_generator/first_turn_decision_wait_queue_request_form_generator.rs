use serde_json::Value as JsonValue;
use crate::game_turn::controller::request_form::first_turn_decision_wait_queue_request_form::FirstTurnDecisionWaitQueueRequestForm;

pub fn create_first_turn_decision_wait_queue_request_form(data: &JsonValue) -> Option<FirstTurnDecisionWaitQueueRequestForm> {
    if let (Some(sessionInfo),  Some(choice)) = (
        data.get("sessionInfo").and_then(|v| v.as_str()),
        data.get("choice").and_then(|v| v.as_str()),

    ) {
        Some(FirstTurnDecisionWaitQueueRequestForm::new(
            sessionInfo.to_string(),
            choice.to_string()))
    } else {
        None
    }
}
