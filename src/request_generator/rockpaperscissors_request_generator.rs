use serde_json::Value as JsonValue;
use crate::rockpaperscissors::controller::request_form::rockpaperscissors_request_form::RockpaperscissorsRequestForm;

pub fn create_rockpaperscissors_request_form(data: &JsonValue) -> Option<RockpaperscissorsRequestForm> {
    if let (Some(sessionInfo),  Some(choice)) = (
        data.get("sessionInfo").and_then(|v| v.as_str()),
        data.get("choice").and_then(|v| v.as_str()),

    ) {
        Some(RockpaperscissorsRequestForm::new(
            sessionInfo.to_string(),
            choice.to_string()))
    } else {
        None
    }
}
