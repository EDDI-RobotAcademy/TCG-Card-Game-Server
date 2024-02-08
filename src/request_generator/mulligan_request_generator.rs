use serde_json::Value as JsonValue;
use crate::game_hand::controller::request_form::mulligan_request_form::MulliganRequestForm;

pub fn create_mulligan_request_form(data: &JsonValue) -> Option<MulliganRequestForm> {
    if let (Some(session_id), Some(hand_card_list)) = (
        data.get("sessionInfo").and_then(|v| v.as_str()),
        data.get("cardList").and_then(|v| v.as_array())
    ) {
        let mut hand_card_list_vector: Vec<String> = Vec::new();
        for card_id_value in hand_card_list {
            if let Some(card_id) = card_id_value.as_str() {
                let card_id_string = card_id.to_string();
                hand_card_list_vector.push(card_id_string);
            }
        }

        Some(MulliganRequestForm::new(session_id.to_string(), hand_card_list_vector))
    } else {
        None
    }
}