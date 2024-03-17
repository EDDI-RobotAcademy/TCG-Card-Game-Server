use serde_json::Value as JsonValue;
use crate::fake_battle_room::controller::request_form::fake_get_nether_from_deck_request_form::FakeGetNetherFromDeckRequestForm;

pub fn create_fake_get_nether_from_deck_request_form(data: &JsonValue) -> Option<FakeGetNetherFromDeckRequestForm> {
    if let (Some(session_info)) = (
        data.get("sessionInfo").and_then(|v| v.as_str())
    ) {
        Some(FakeGetNetherFromDeckRequestForm::new(session_info.to_string()))
    } else {
        None
    }
}
