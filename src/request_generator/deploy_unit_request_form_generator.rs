use serde_json::Value as JsonValue;
use crate::game_card_unit::controller::request_form::deploy_unit_request_form::DeployUnitRequestForm;

pub fn create_deploy_unit_request_form(data: &JsonValue) -> Option<DeployUnitRequestForm> {
    if let (Some(unit_number), Some(session_info)) = (
        data.get("unitId").and_then(|v| v.as_str()),
        data.get("sessionInfo").and_then(|v| v.as_str())
    ) {
        Some(DeployUnitRequestForm::new(session_info.to_string(), unit_number.to_string()))
    } else {
        None
    }
}
