use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use crate::ui_data_generator::entity::field_unit_health_point_info::FieldUnitHealthPointInfo;
use crate::ui_data_generator::entity::player_index_enum::PlayerIndex;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenerateOpponentSpecificUnitHealthPointDataResponse {
    player_field_unit_health_point_map_for_response: HashMap<PlayerIndex, FieldUnitHealthPointInfo>,
    player_field_unit_health_point_map_for_notice: HashMap<PlayerIndex, FieldUnitHealthPointInfo>,
}

impl GenerateOpponentSpecificUnitHealthPointDataResponse {
    pub fn new(player_field_unit_health_point_map_for_response: HashMap<PlayerIndex, FieldUnitHealthPointInfo>,
               player_field_unit_health_point_map_for_notice: HashMap<PlayerIndex, FieldUnitHealthPointInfo>,
    ) -> Self {
        GenerateOpponentSpecificUnitHealthPointDataResponse {
            player_field_unit_health_point_map_for_response,
            player_field_unit_health_point_map_for_notice,
        }
    }

    pub fn get_player_field_unit_health_point_map_for_response(&self) -> &HashMap<PlayerIndex, FieldUnitHealthPointInfo> {
        &self.player_field_unit_health_point_map_for_response
    }

    pub fn get_player_field_unit_health_point_map_for_notice(&self) -> &HashMap<PlayerIndex, FieldUnitHealthPointInfo> {
        &self.player_field_unit_health_point_map_for_notice
    }
}