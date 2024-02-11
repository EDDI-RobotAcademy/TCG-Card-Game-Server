use serde_json::Value as JsonValue;
use crate::first_turn_decision_wait_queue::service::request::first_turn_decision_wait_queue_request::FirstTurnDecisionWaitQueueRequest;

pub fn create_first_turn_decision_wait_queue_request(data: &JsonValue) -> Option<FirstTurnDecisionWaitQueueRequest> {
    if let (Some(sessionInfo),  Some(choice)) = (
        data.get("sessionInfo").and_then(|v| v.as_str()),
        data.get("choice").and_then(|v| v.as_str()),

    ) {
        Some(FirstTurnDecisionWaitQueueRequest::new(
            sessionInfo.to_string(),
            choice.to_string()))
    } else {
        None
    }
}
