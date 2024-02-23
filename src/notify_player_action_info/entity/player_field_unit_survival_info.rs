use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use crate::notify_player_action_info::entity::field_unit_survival_info::FieldUnitSurvivalInfo;
use crate::notify_player_action_info::entity::player_index_enum::PlayerIndex;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PlayerFieldUnitSurvivalInfo {
    player_field_unit_survival_map: HashMap<PlayerIndex, FieldUnitSurvivalInfo>,
}

impl PlayerFieldUnitSurvivalInfo {
    pub fn new(player_field_unit_survival_map: HashMap<PlayerIndex, FieldUnitSurvivalInfo>) -> Self {
        PlayerFieldUnitSurvivalInfo {
            player_field_unit_survival_map
        }
    }
}