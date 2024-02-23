use serde_json::Value as JsonValue;
use crate::battle_field_info::service::request::remain_deck_card_count_request::RemainDeckCardCountRequest;


pub fn create_remain_deck_card_count_request(data: &JsonValue) -> Option<RemainDeckCardCountRequest> {
    if let (Some(sessionInfo), Some(who)  ) = (
        data.get("sessionInfo").and_then(|v| v.as_str()),
        data.get("who").and_then(|v| v.as_str())

    ) {
        Some(RemainDeckCardCountRequest::new(
            sessionInfo.to_string(),
            who.to_string(),
            ))
    } else {
        None
    }
}
