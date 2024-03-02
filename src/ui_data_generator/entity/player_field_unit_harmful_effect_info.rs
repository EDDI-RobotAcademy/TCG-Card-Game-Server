use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use crate::ui_data_generator::entity::field_unit_harmful_status_info::FieldUnitHarmfulStatusInfo;
use crate::ui_data_generator::entity::player_index_enum::PlayerIndex;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PlayerFieldUnitHarmfulEffectInfo {
    player_field_unit_harmful_effect_map: HashMap<PlayerIndex, FieldUnitHarmfulStatusInfo>,
}

impl PlayerFieldUnitHarmfulEffectInfo {
    pub fn new(player_field_unit_harmful_effect_map: HashMap<PlayerIndex, FieldUnitHarmfulStatusInfo>) -> Self {
        PlayerFieldUnitHarmfulEffectInfo {
            player_field_unit_harmful_effect_map
        }
    }
    pub fn get_player_field_unit_harmful_effect_map(&self) -> &HashMap<PlayerIndex, FieldUnitHarmfulStatusInfo> {
        &self.player_field_unit_harmful_effect_map
    }
}