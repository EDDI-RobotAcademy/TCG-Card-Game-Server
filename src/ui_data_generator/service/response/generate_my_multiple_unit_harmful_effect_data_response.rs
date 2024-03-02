use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use crate::ui_data_generator::entity::field_unit_harmful_status_info::FieldUnitHarmfulStatusInfo;
use crate::ui_data_generator::entity::player_index_enum::PlayerIndex;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenerateMyMultipleUnitHarmfulEffectDataResponse {
    player_field_unit_harmful_effect_map_for_response: HashMap<PlayerIndex, FieldUnitHarmfulStatusInfo>,
    player_field_unit_harmful_effect_map_for_notice: HashMap<PlayerIndex, FieldUnitHarmfulStatusInfo>,
}

impl GenerateMyMultipleUnitHarmfulEffectDataResponse {
    pub fn new(player_field_unit_harmful_effect_map_for_response: HashMap<PlayerIndex, FieldUnitHarmfulStatusInfo>,
               player_field_unit_harmful_effect_map_for_notice: HashMap<PlayerIndex, FieldUnitHarmfulStatusInfo>,
    ) -> Self {
        GenerateMyMultipleUnitHarmfulEffectDataResponse {
            player_field_unit_harmful_effect_map_for_response,
            player_field_unit_harmful_effect_map_for_notice,
        }
    }

    pub fn get_player_field_unit_harmful_effect_map_for_response(&self) -> &HashMap<PlayerIndex, FieldUnitHarmfulStatusInfo> {
        &self.player_field_unit_harmful_effect_map_for_response
    }

    pub fn get_player_field_unit_harmful_effect_map_for_notice(&self) -> &HashMap<PlayerIndex, FieldUnitHarmfulStatusInfo> {
        &self.player_field_unit_harmful_effect_map_for_notice
    }
}