use serde_json::Value as JsonValue;
use crate::game_card_item::controller::request_form::add_field_energy_with_field_unit_health_point_item_request_form::AddFieldEnergyWithFieldUnitHealthPointRequestForm;

pub fn create_add_field_energy_by_field_unit_health_point_request_form(data: &JsonValue) -> Option<AddFieldEnergyWithFieldUnitHealthPointRequestForm> {
    if let (Some(unit_index_number), Some(session_info), Some(item_card_id)) = (
        data.get("unitIndex").and_then(|v| v.as_str()),
        data.get("sessionInfo").and_then(|v| v.as_str()),
        data.get("itemCardId").and_then(|v| v.as_str()),
    ) {
        Some(AddFieldEnergyWithFieldUnitHealthPointRequestForm::new(session_info, item_card_id, unit_index_number))
    } else {
        None
    }
}