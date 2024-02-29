use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use crate::ui_data_generator::entity::field_unit_extra_effect_info::FieldUnitExtraEffectInfo;
use crate::ui_data_generator::entity::player_index_enum::PlayerIndex;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenerateMyMultipleUnitExtraEffectDataResponse {
    player_field_unit_extra_effect_map_for_response: HashMap<PlayerIndex, FieldUnitExtraEffectInfo>,
    player_field_unit_extra_effect_map_for_notice: HashMap<PlayerIndex, FieldUnitExtraEffectInfo>,
}

impl GenerateMyMultipleUnitExtraEffectDataResponse {
    pub fn new(player_field_unit_extra_effect_map_for_response: HashMap<PlayerIndex, FieldUnitExtraEffectInfo>,
               player_field_unit_extra_effect_map_for_notice: HashMap<PlayerIndex, FieldUnitExtraEffectInfo>,
    ) -> Self {
        GenerateMyMultipleUnitExtraEffectDataResponse {
            player_field_unit_extra_effect_map_for_response,
            player_field_unit_extra_effect_map_for_notice,
        }
    }

    pub fn get_player_field_unit_extra_effect_map_for_response(&self) -> &HashMap<PlayerIndex, FieldUnitExtraEffectInfo> {
        &self.player_field_unit_extra_effect_map_for_response
    }

    pub fn get_player_field_unit_extra_effect_map_for_notice(&self) -> &HashMap<PlayerIndex, FieldUnitExtraEffectInfo> {
        &self.player_field_unit_extra_effect_map_for_notice
    }
}