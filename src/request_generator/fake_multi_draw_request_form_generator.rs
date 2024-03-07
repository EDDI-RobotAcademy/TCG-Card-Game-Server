use serde_json::Value as JsonValue;
use crate::fake_battle_room::controller::request_form::fake_multi_draw_request_form::FakeMultiDrawRequestForm;

pub fn create_fake_multi_draw_request_form(data: &JsonValue) -> Option<FakeMultiDrawRequestForm> {
    if let (Some(session_info)) = (
        data.get("sessionInfo").and_then(|v| v.as_str())
    ) {
        Some(FakeMultiDrawRequestForm::new(session_info.to_string()))
    } else {
        None
    }
}
