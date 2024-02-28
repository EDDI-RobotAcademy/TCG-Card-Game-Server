use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use crate::ui_data_generator::entity::field_unit_energy_info::FieldUnitEnergyInfo;
use crate::ui_data_generator::entity::player_field_unit_energy_info::PlayerFieldUnitEnergyInfo;
use crate::ui_data_generator::entity::player_index_enum::PlayerIndex;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NoticeUseGeneralEnergyCardToMySpecificUnitResponse {
    player_field_unit_energy_map: HashMap<PlayerIndex, FieldUnitEnergyInfo>,
}

impl NoticeUseGeneralEnergyCardToMySpecificUnitResponse {
    pub fn new(player_field_unit_energy_map: HashMap<PlayerIndex, FieldUnitEnergyInfo>) -> Self {

        NoticeUseGeneralEnergyCardToMySpecificUnitResponse {
            player_field_unit_energy_map
        }
    }

    pub fn from_info(player_field_unit_energy_info: PlayerFieldUnitEnergyInfo
    ) -> NoticeUseGeneralEnergyCardToMySpecificUnitResponse {

        NoticeUseGeneralEnergyCardToMySpecificUnitResponse::new(
            player_field_unit_energy_info
                .get_player_field_unit_energy_map().clone())
    }

    pub fn get_player_field_unit_energy_map(&self) -> &HashMap<PlayerIndex, FieldUnitEnergyInfo> {
        &self.player_field_unit_energy_map
    }
}