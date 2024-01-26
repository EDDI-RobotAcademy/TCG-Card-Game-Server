use serde_json::Value as JsonValue;
use crate::deck_card::service::request::deck_configuration_request::DeckConfigurationRequest;

pub fn create_deck_configuration_request(data: &JsonValue) -> Option<DeckConfigurationRequest> {
    if let (Some(deck_id), Some(card_list)) = (
        data.get("deckId").and_then(|v| v.as_i64()),
        data.get("cardIdList").and_then(|v| v.as_array()),
    ) {
        let deck_id_i32 = deck_id as i32;
        let mut card_vec_i32 = Vec::new();

        for card_id_value  in card_list.iter() {
            if let Some(card_id) = card_id_value.as_i64() {
                let card_id_i32 = card_id as i32;
                card_vec_i32.push(card_id_i32);
            }
        }
        Some(DeckConfigurationRequest::new(deck_id_i32, card_vec_i32))
    } else {
        None
    }
}