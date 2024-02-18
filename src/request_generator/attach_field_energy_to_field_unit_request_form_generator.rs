use serde_json::Value as JsonValue;
use crate::game_field_energy::controller::request_form::attach_field_energy_to_field_unit_request_form::AttachFieldEnergyToFieldUnitRequestForm;

pub fn create_attach_field_energy_to_field_unit_request_form(data: &JsonValue) -> Option<AttachFieldEnergyToFieldUnitRequestForm> {
    if let (Some(session_info), Some(unit_card_index), Some(energy_race), Some(energy_count)) = (
        data.get("unitIndex").and_then(|v| v.as_str()),
        data.get("sessionInfo").and_then(|v| v.as_str()),
        data.get("energyRace").and_then(|v| v.as_str()),
        data.get("energyCount").and_then(|v| v.as_str()),
    ) {
        Some(AttachFieldEnergyToFieldUnitRequestForm::new(session_info,
                                                          unit_card_index,
                                                          energy_race,
                                                          energy_count))
    } else {
        None
    }
}