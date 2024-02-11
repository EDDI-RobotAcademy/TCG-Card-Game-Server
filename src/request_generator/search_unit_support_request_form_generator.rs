use serde_json::Value as JsonValue;
use crate::game_card_support::controller::request_form::search_unit_support_request_form::SearchUnitSupportRequestForm;

pub fn create_search_unit_support_request_form(data: &JsonValue) -> Option<SearchUnitSupportRequestForm> {
    if let (Some(session_id), Some(support_card_id), Some(target_unit_id_list)) = (
        data.get("sessionInfo").and_then(|v| v.as_str()),
        data.get("supportCardId").and_then(|v| v.as_str()),
        data.get("targetUnitCardList").and_then(|v| v.as_array())
    ) {
        let mut target_unit_card_list: Vec<String> = Vec::new();
        for unit_card_id_value in target_unit_id_list {
            let unit_card_id_string = unit_card_id_value.to_string();
            target_unit_card_list.push(unit_card_id_string);
        }
        Some(SearchUnitSupportRequestForm::new(session_id, support_card_id, target_unit_card_list))
    } else {
        None
    }
}