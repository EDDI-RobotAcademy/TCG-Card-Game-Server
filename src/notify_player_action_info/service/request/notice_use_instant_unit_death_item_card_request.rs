use std::collections::HashMap;
use crate::ui_data_generator::entity::field_unit_death_info::FieldUnitDeathInfo;
use crate::ui_data_generator::entity::field_unit_health_point_info::FieldUnitHealthPointInfo;
use crate::ui_data_generator::entity::player_index_enum::PlayerIndex;
use crate::ui_data_generator::entity::used_hand_card_info::UsedHandCardInfo;

#[derive(Debug)]
pub struct NoticeUseInstantUnitDeathItemCardRequest {
    opponent_unique_id: i32,
    player_hand_use_map_for_notice: HashMap<PlayerIndex, UsedHandCardInfo>,
    player_field_unit_health_point_map_for_notice: HashMap<PlayerIndex, FieldUnitHealthPointInfo>,
    player_field_unit_death_map_for_notice: HashMap<PlayerIndex, FieldUnitDeathInfo>,
}

impl NoticeUseInstantUnitDeathItemCardRequest {
    pub fn new(opponent_unique_id: i32,
               player_hand_use_map_for_notice: HashMap<PlayerIndex, UsedHandCardInfo>,
               player_field_unit_health_point_map_for_notice: HashMap<PlayerIndex, FieldUnitHealthPointInfo>,
               player_field_unit_death_map_for_notice: HashMap<PlayerIndex, FieldUnitDeathInfo>,) -> Self {
        NoticeUseInstantUnitDeathItemCardRequest {
            opponent_unique_id,
            player_hand_use_map_for_notice,
            player_field_unit_health_point_map_for_notice,
            player_field_unit_death_map_for_notice
        }
    }

    pub fn get_opponent_unique_id(&self) -> i32 { self.opponent_unique_id }

    pub fn get_player_hand_use_map_for_notice(&self) -> &HashMap<PlayerIndex, UsedHandCardInfo> {
        &self.player_hand_use_map_for_notice
    }

    pub fn get_player_field_unit_health_point_map_for_notice(&self) -> &HashMap<PlayerIndex, FieldUnitHealthPointInfo> {
        &self.player_field_unit_health_point_map_for_notice
    }

    pub fn get_player_field_unit_death_map_for_notice(&self) -> &HashMap<PlayerIndex, FieldUnitDeathInfo> {
        &self.player_field_unit_death_map_for_notice
    }
}