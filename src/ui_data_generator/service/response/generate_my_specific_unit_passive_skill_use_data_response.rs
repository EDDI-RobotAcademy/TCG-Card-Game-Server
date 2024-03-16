use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use crate::ui_data_generator::entity::field_unit_basic_attack_info::FieldUnitAttackInfo;
use crate::ui_data_generator::entity::player_index_enum::PlayerIndex;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenerateMySpecificUnitPassiveSkillUseDataResponse {
    player_field_unit_attack_map_for_response: HashMap<PlayerIndex, FieldUnitAttackInfo>,
    player_field_unit_attack_map_for_notice: HashMap<PlayerIndex, FieldUnitAttackInfo>,
}

impl GenerateMySpecificUnitPassiveSkillUseDataResponse {
    pub fn new(
        player_field_unit_attack_map_for_response: HashMap<PlayerIndex, FieldUnitAttackInfo>,
        player_field_unit_attack_map_for_notice: HashMap<PlayerIndex, FieldUnitAttackInfo>,) -> Self {
        GenerateMySpecificUnitPassiveSkillUseDataResponse {
            player_field_unit_attack_map_for_response,
            player_field_unit_attack_map_for_notice,
        }
    }

    pub fn get_player_field_unit_attack_map_for_response(&self) -> &HashMap<PlayerIndex, FieldUnitAttackInfo> {
        &self.player_field_unit_attack_map_for_response
    }

    pub fn get_player_field_unit_attack_map_for_notice(&self) -> &HashMap<PlayerIndex, FieldUnitAttackInfo> {
        &self.player_field_unit_attack_map_for_notice
    }
}