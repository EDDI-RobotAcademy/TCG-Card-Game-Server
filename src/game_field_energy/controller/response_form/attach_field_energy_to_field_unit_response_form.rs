use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use crate::ui_data_generator::entity::field_unit_energy_info::FieldUnitEnergyInfo;
use crate::ui_data_generator::entity::player_index_enum::PlayerIndex;
use crate::notify_player_action_info::service::response::notice_use_field_energy_to_specific_unit_response::NoticeUseFieldEnergyToSpecificUnitResponse;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AttachFieldEnergyToFieldUnitResponseForm {
    is_success: bool,
    player_field_unit_energy_map: HashMap<PlayerIndex, FieldUnitEnergyInfo>,
    player_field_energy_map: HashMap<PlayerIndex, i32>,
}

impl AttachFieldEnergyToFieldUnitResponseForm {
    pub fn new(is_success: bool,
               player_field_unit_energy_map: HashMap<PlayerIndex, FieldUnitEnergyInfo>,
               player_field_energy_map: HashMap<PlayerIndex, i32>) -> Self {
        AttachFieldEnergyToFieldUnitResponseForm {
            is_success,
            player_field_unit_energy_map,
            player_field_energy_map,
        }
    }
    pub fn from_response(notice_use_field_energy_to_specific_unit_response: NoticeUseFieldEnergyToSpecificUnitResponse) -> AttachFieldEnergyToFieldUnitResponseForm {
        AttachFieldEnergyToFieldUnitResponseForm::new(
            true,
            notice_use_field_energy_to_specific_unit_response
                .get_player_field_unit_energy_info()
                .get_player_field_unit_energy_map().clone(),
            notice_use_field_energy_to_specific_unit_response
                .get_player_field_energy_info()
                .get_player_field_energy_map().clone())
    }

    pub fn default() -> AttachFieldEnergyToFieldUnitResponseForm {
        AttachFieldEnergyToFieldUnitResponseForm::new(
            false,
            HashMap::new(),
            HashMap::new())
    }
}