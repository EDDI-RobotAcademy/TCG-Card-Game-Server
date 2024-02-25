use serde_json::Value as JsonValue;
use crate::fake_battle_room::controller::request_form::create_fake_battle_room_request_form::CreateFakeBattleRoomRequestForm;

pub fn create_fake_battle_room_create_request_form(data: &JsonValue) -> Option<CreateFakeBattleRoomRequestForm> {
    if let (Some(first_fake_test_account_id),
            Some(first_fake_test_account_password),
            Some(second_fake_test_account_id),
            Some(second_fake_test_account_password)) = (
        data.get("firstFakeTestAccountId").and_then(|v| v.as_str()),
        data.get("firstFakeTestPassword").and_then(|v| v.as_str()),
        data.get("secondFakeTestAccountId").and_then(|v| v.as_str()),
        data.get("secondFakeTestPassword").and_then(|v| v.as_str()),
    ) {
        Some(CreateFakeBattleRoomRequestForm::new(
            first_fake_test_account_id.to_string(),
            first_fake_test_account_password.to_string(),
            second_fake_test_account_id.to_string(),
            second_fake_test_account_password.to_string()))
    } else {
        None
    }
}
