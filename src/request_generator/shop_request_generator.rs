use serde_json::Value as JsonValue;
use crate::game_turn::controller::request_form::turn_end_request_form::TurnEndRequestForm;
use crate::shop::controller::request_form::execute_free_gacha_request_form::ExecuteFreeGachaRequestForm;
use crate::shop::controller::request_form::execute_shop_gacha_request_form::ExecuteShopGachaRequestForm;
use crate::shop::service::request::data_to_display_in_shop_request::DataToDisplayInShopRequest;


pub fn create_data_to_display_in_shop_request(data: &JsonValue) -> Option<DataToDisplayInShopRequest> {
    if let (Some(sessionInfo),) = (
        data.get("sessionInfo").and_then(|v| v.as_str()),
    ) {
        Some(DataToDisplayInShopRequest::new(sessionInfo.to_string()))
    } else {
        None
    }
}
pub fn create_execute_shop_gacha_request_form(data: &JsonValue) -> Option<ExecuteShopGachaRequestForm> {
    if let (Some(account_session_id), Some(race_name), Some(is_confirmed_upper_legend)) = (
        data.get("sessionInfo").and_then(|v| v.as_str()),
        data.get("race_name").and_then(|v| v.as_str()),
        data.get("is_confirmed_upper_legend").and_then(|v| v.as_bool()))
    {
        Some(ExecuteShopGachaRequestForm::new(account_session_id.to_string(), race_name.to_string(), is_confirmed_upper_legend))
    } else {
        None
    }
}
pub fn create_execute_free_gacha_request_form(data: &JsonValue) -> Option<ExecuteFreeGachaRequestForm> {
    if let (Some(account_session_id), Some(race_name), Some(is_confirmed_upper_legend)) = (
        data.get("sessionInfo").and_then(|v| v.as_str()),
        data.get("race_name").and_then(|v| v.as_str()),
        data.get("is_confirmed_upper_legend").and_then(|v| v.as_bool()))
    {
        Some(ExecuteFreeGachaRequestForm::new(account_session_id.to_string(), race_name.to_string(), is_confirmed_upper_legend))
    } else {
        None
    }
}