use serde_json::Value as JsonValue;
use crate::game_card_item::controller::request_form::add_field_energy_with_field_unit_health_point_item_request_form::AddFieldEnergyWithFieldUnitHealthPointRequestForm;
use crate::game_card_item::controller::request_form::catastrophic_damage_item_request_form::CatastrophicDamageItemRequestForm;
use crate::game_card_item::controller::request_form::multiple_target_damage_by_field_unit_death_item_request_form::MultipleTargetDamageByFieldUnitDeathItemRequestForm;
use crate::game_card_item::controller::request_form::target_death_item_request_form::TargetDeathItemRequestForm;

pub fn create_add_field_energy_by_field_unit_health_point_item_request_form(data: &JsonValue) -> Option<AddFieldEnergyWithFieldUnitHealthPointRequestForm> {
    if let (Some(unit_index_number), Some(session_info), Some(item_card_id)) = (
        data.get("unitIndex").and_then(|v| v.as_str()),
        data.get("sessionInfo").and_then(|v| v.as_str()),
        data.get("itemCardId").and_then(|v| v.as_str()),
    ) {
        Some(AddFieldEnergyWithFieldUnitHealthPointRequestForm::new(session_info,
                                                                    item_card_id,
                                                                    unit_index_number))
    } else {
        None
    }
}

pub fn create_target_death_item_request_form(data: &JsonValue) -> Option<TargetDeathItemRequestForm> {
    if let (Some(opponent_target_unit_index_number), Some(session_info), Some(item_card_id)) = (
        data.get("opponentTargetUnitIndex").and_then(|v| v.as_str()),
        data.get("sessionInfo").and_then(|v| v.as_str()),
        data.get("itemCardId").and_then(|v| v.as_str()),
    ) {
        Some(TargetDeathItemRequestForm::new(session_info.to_string(),
                                             opponent_target_unit_index_number.to_string(),
                                             item_card_id.to_string()))
    } else {
        None
    }
}

pub fn create_catastrophic_damage_item_request_form(data: &JsonValue) -> Option<CatastrophicDamageItemRequestForm> {
    if let (Some(session_info), Some(item_card_id)) = (
        data.get("sessionInfo").and_then(|v| v.as_str()),
        data.get("itemCardId").and_then(|v| v.as_str()),
    ) {
        Some(CatastrophicDamageItemRequestForm::new(session_info, item_card_id))
    } else {
        None
    }
}

pub fn create_multiple_target_damage_by_field_unit_sacrifice_item(data: &JsonValue) -> Option<MultipleTargetDamageByFieldUnitDeathItemRequestForm> {
    if let (Some(opponent_target_unit_index_list), Some(session_info), Some(item_card_id), Some(my_field_unit_index)) = (
        data.get("opponentTargetUnitIndexList").and_then(|v| v.as_array()),
        data.get("sessionInfo").and_then(|v| v.as_str()),
        data.get("itemCardId").and_then(|v| v.as_str()),
        data.get("unitIndexToSacrifice").and_then(|v| v.as_str()),
    ) {
        let mut opponent_target_unit_index_list_string = Vec::new();
        for value in opponent_target_unit_index_list {
            let index = value.to_string();
            opponent_target_unit_index_list_string.push(index);
        }
        Some(MultipleTargetDamageByFieldUnitDeathItemRequestForm::new(my_field_unit_index,
                                                                      session_info,
                                                                      item_card_id,
                                                                      opponent_target_unit_index_list_string))
    } else {
        None
    }
}