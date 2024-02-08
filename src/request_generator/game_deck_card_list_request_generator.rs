use serde_json::Value as JsonValue;
use crate::game_deck::service::request::game_deck_start_card_list_request::{GameDeckStartCardListRequest};

pub fn create_game_deck_card_list_request(data: &JsonValue) -> Option<GameDeckStartCardListRequest> {
    if let (Some(deck_id), Some(session_info)) = (
        data.get("deckId").and_then(|v| v.as_str()),
        data.get("sessionInfo").and_then(|v| v.as_str())
    ) {
        Some(GameDeckStartCardListRequest::new(deck_id.to_string(), session_info.to_string()))
    } else {
        None
    }
}
