use serde_json::Value as JsonValue;
use crate::game_deck::service::request::game_deck_card_list_request::GameDeckCardListRequest;

pub fn create_game_deck_card_list_request(data: &JsonValue) -> Option<GameDeckCardListRequest> {
    if let (Some(deck_id), Some(session_info)) = (
        data.get("deckId").and_then(|v| v.as_i64()),
        data.get("sessionInfo").and_then(|v| v.as_str())
    ) {
        let deck_id_i32 = deck_id as i32;
        Some(GameDeckCardListRequest::new(deck_id_i32, session_info.to_string()))
    } else {
        None
    }
}
