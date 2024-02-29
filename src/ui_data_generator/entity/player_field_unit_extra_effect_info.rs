use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use crate::ui_data_generator::entity::field_unit_extra_effect_info::FieldUnitExtraEffectInfo;
use crate::ui_data_generator::entity::player_index_enum::PlayerIndex;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PlayerFieldUnitExtraEffectInfo {
    player_field_unit_extra_effect_map: HashMap<PlayerIndex, FieldUnitExtraEffectInfo>,
}

impl PlayerFieldUnitExtraEffectInfo {
    pub fn new(player_field_unit_extra_effect_map: HashMap<PlayerIndex, FieldUnitExtraEffectInfo>) -> Self {
        PlayerFieldUnitExtraEffectInfo {
            player_field_unit_extra_effect_map
        }
    }
    pub fn get_player_field_unit_extra_effect_map(&self) -> &HashMap<PlayerIndex, FieldUnitExtraEffectInfo> {
        &self.player_field_unit_extra_effect_map
    }
}