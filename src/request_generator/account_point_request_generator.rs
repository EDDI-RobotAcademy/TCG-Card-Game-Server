use serde_json::Value as JsonValue;
use crate::account_point::service::request::gain_gold_request::GainGoldRequest;

pub fn create_gain_gold_request(data: &JsonValue) -> Option<GainGoldRequest> {
    if let (Some(account_id), Some(gold)) = (
        data.get("accountId").and_then(|v| v.as_i64()),
        data.get("gold").and_then(|v| v.as_i64())
    ) {
        let account_id_i32 = account_id as i32;
        let gold_i32 = gold as i32;
        Some(GainGoldRequest::new(account_id_i32, gold_i32))
    } else {
        None
    }
}