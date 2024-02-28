use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use crate::ui_data_generator::entity::field_unit_health_point_info::FieldUnitHealthPointInfo;
use crate::ui_data_generator::entity::player_index_enum::PlayerIndex;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PlayerFieldUnitHealthPointInfo {
    player_field_unit_health_point_map: HashMap<PlayerIndex, FieldUnitHealthPointInfo>,
}

impl PlayerFieldUnitHealthPointInfo {
    pub fn new(player_field_unit_health_point_map: HashMap<PlayerIndex, FieldUnitHealthPointInfo>,) -> Self {
        PlayerFieldUnitHealthPointInfo {
            player_field_unit_health_point_map
        }
    }
    pub fn get_player_field_unit_health_point_map(&self) -> &HashMap<PlayerIndex, FieldUnitHealthPointInfo> {
        &self.player_field_unit_health_point_map
    }
}