use serde_json::Value as JsonValue;
use crate::game_card_support::controller::request_form::check_search_unit_support_available_request_form::CheckSearchUnitSupportAvailableRequestForm;
use crate::game_card_support::controller::request_form::search_unit_support_request_form::SearchUnitSupportRequestForm;

pub fn create_search_unit_support_request_form(data: &JsonValue) -> Option<SearchUnitSupportRequestForm> {
    if let (Some(session_id), Some(support_card_id), Some(target_unit_index_list)) = (
        data.get("sessionInfo").and_then(|v| v.as_str()),
        data.get("supportCardId").and_then(|v| v.as_str()),
        data.get("targetUnitCardIndexList").and_then(|v| v.as_array())
    ) {
        let mut target_unit_card_list: Vec<String> = Vec::new();
        for unit_card_index_value in target_unit_index_list {
            if let Some(unit_card_id_str) = unit_card_index_value.as_str() {
                let unit_card_id_string = unit_card_id_str.to_string();
                target_unit_card_list.push(unit_card_id_string);
            }
        }
        Some(SearchUnitSupportRequestForm::new(session_id, support_card_id, target_unit_card_list))
    } else {
        None
    }
}

pub fn create_check_search_unit_support_available_request_form(data: &JsonValue) -> Option<CheckSearchUnitSupportAvailableRequestForm> {
    if let (Some(session_id), Some(support_card_id)) = (
        data.get("sessionInfo").and_then(|v| v.as_str()),
        data.get("supportCardId").and_then(|v| v.as_str()),
    ) {
        Some(CheckSearchUnitSupportAvailableRequestForm::new(session_id, support_card_id))
    } else {
        None
    }
}