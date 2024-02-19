use serde_json::Value as JsonValue;
use crate::rockpaperscissors::controller::request_form::check_winner_request_form::CheckWinnerRequestForm;

pub fn create_check_winner_request_form(data: &JsonValue) -> Option<CheckWinnerRequestForm> {
    if let (Some(sessionInfo), ) = (
        data.get("sessionInfo").and_then(|v| v.as_str()),


    ) {
        Some(CheckWinnerRequestForm::new(
            sessionInfo.to_string(),
            ))
    } else {
        None
    }
}
