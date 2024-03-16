use serde_json::Value as JsonValue;
use crate::fake_battle_room::controller::request_form::fake_get_all_cards_of_specific_kind_from_deck_request_form::FakeGetAllCardsOfSpecificKindFromDeckRequestForm;

pub fn create_fake_get_all_specific_kind_cards_from_deck_request_form(data: &JsonValue) -> Option<FakeGetAllCardsOfSpecificKindFromDeckRequestForm> {
    if let (Some(session_info), Some(card_kind_index)) = (
        data.get("sessionInfo").and_then(|v| v.as_str()),
        data.get("card_kind_index").and_then(|v| v.as_str()),
    ) {
        Some(FakeGetAllCardsOfSpecificKindFromDeckRequestForm::new(session_info.to_string(), card_kind_index.to_string()))
    } else {
        None
    }
}
