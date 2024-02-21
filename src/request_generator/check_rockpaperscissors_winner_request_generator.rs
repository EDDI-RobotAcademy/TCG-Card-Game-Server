use serde_json::Value as JsonValue;
use crate::rockpaperscissors::controller::request_form::check_rockpaperscissors_winner_request_form::CheckRockpaperscissorsWinnerRequestForm;

pub fn create_check_rockpaperscissors_winner_request_form(data: &JsonValue) -> Option<CheckRockpaperscissorsWinnerRequestForm> {
    if let (Some(sessionInfo), ) = (
        data.get("sessionInfo").and_then(|v| v.as_str()),


    ) {
        Some(CheckRockpaperscissorsWinnerRequestForm::new(
            sessionInfo.to_string(),
            ))
    } else {
        None
    }
}
