use serde_json::Value as JsonValue;
use crate::rock_paper_scissors::controller::request_form::check_rock_paper_scissors_winner_request_form::CheckRockPaperScissorsWinnerRequestForm;

pub fn create_check_rockpaperscissors_winner_request_form(data: &JsonValue) -> Option<CheckRockPaperScissorsWinnerRequestForm> {
    if let (Some(sessionInfo), ) = (
        data.get("sessionInfo").and_then(|v| v.as_str()),


    ) {
        Some(CheckRockPaperScissorsWinnerRequestForm::new(
            sessionInfo.to_string(),
            ))
    } else {
        None
    }
}
