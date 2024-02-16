use serde_json::Value as JsonValue;

use crate::shop_gacha::service::request::get_specific_race_card_request::GetSpecificRaceCardRequest;


pub fn create_get_specific_race_card_default_request(data: &JsonValue) -> Option<GetSpecificRaceCardRequest> {
    if let (Some(account_session_id), Some(race_name), Some(is_confirmed_upper_legend)) = (
        data.get("sessionInfo").and_then(|v| v.as_str()),
        data.get("race_name").and_then(|v| v.as_str()),
        data.get("is_confirmed_upper_legend").and_then(|v| v.as_bool()))
    {
        Some(GetSpecificRaceCardRequest::new(account_session_id.to_string(), race_name.to_string(), is_confirmed_upper_legend))
    } else {
        None
    }
}