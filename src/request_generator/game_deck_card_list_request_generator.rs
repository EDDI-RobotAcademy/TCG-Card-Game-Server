use serde_json::Value as JsonValue;
use crate::game_deck::service::request::game_start_deck_card_list_request::{GameStartDeckCardListRequest};

pub fn create_game_deck_card_list_request(data: &JsonValue) -> Option<GameStartDeckCardListRequest> {
    if let (Some(deck_id), Some(session_info)) = (
        data.get("deckId").and_then(|v| v.as_str()),
        data.get("sessionInfo").and_then(|v| v.as_str())
    ) {
        Some(GameStartDeckCardListRequest::new(deck_id.to_string(), session_info.to_string()))
    } else {
        None
    }
}
