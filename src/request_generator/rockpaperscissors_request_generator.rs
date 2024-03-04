use serde_json::Value as JsonValue;
use crate::rock_paper_scissors::controller::request_form::rock_paper_scissors_request_form::RockPaperScissorsRequestForm;

pub fn create_rockpaperscissors_request_form(data: &JsonValue) -> Option<RockPaperScissorsRequestForm> {
    if let (Some(sessionInfo),  Some(choice)) = (
        data.get("sessionInfo").and_then(|v| v.as_str()),
        data.get("choice").and_then(|v| v.as_str()),

    ) {
        Some(RockPaperScissorsRequestForm::new(
            sessionInfo.to_string(),
            choice.to_string()))
    } else {
        None
    }
}
