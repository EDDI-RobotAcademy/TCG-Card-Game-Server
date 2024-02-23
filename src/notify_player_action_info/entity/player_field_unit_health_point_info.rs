use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use crate::notify_player_action_info::entity::field_unit_health_point_info::FieldUnitHealthPointInfo;
use crate::notify_player_action_info::entity::player_index_enum::PlayerIndex;

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
}