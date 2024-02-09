use serde_json::Value as JsonValue;
use crate::game_card_support::controller::request_form::draw_support_request_form::DrawSupportRequestForm;

pub fn create_general_draw_support_request_form(data: &JsonValue) -> Option<DrawSupportRequestForm> {
    if let (Some(session_id), Some(support_card_id)) = (
        data.get("sessionInfo").and_then(|v| v.as_str()),
        data.get("supportCardId").and_then(|v| v.as_str())
    ) {
        Some(DrawSupportRequestForm::new(session_id, support_card_id))
    } else {
        None
    }
}