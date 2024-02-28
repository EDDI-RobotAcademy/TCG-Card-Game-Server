use serde::{Deserialize, Serialize};
use crate::ui_data_generator::entity::player_field_unit_damage_info::PlayerFieldUnitDamageInfo;
use crate::ui_data_generator::entity::player_field_unit_death_info::PlayerFieldUnitDeathInfo;
use crate::ui_data_generator::entity::player_field_unit_health_point_info::PlayerFieldUnitHealthPointInfo;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NoticeApplyDamageToMultipleOpponentUnitResponse {
    player_field_unit_damage_info: PlayerFieldUnitDamageInfo,
    player_field_unit_health_point_info: PlayerFieldUnitHealthPointInfo,
    player_field_unit_death_info: PlayerFieldUnitDeathInfo
}

impl NoticeApplyDamageToMultipleOpponentUnitResponse {
    pub fn new(player_field_unit_damage_info: PlayerFieldUnitDamageInfo,
               player_field_unit_health_point_info: PlayerFieldUnitHealthPointInfo,
               player_field_unit_death_info: PlayerFieldUnitDeathInfo) -> Self {
        NoticeApplyDamageToMultipleOpponentUnitResponse {
            player_field_unit_damage_info,
            player_field_unit_health_point_info,
            player_field_unit_death_info
        }
    }

    pub fn get_player_field_unit_damage_info(&self) -> &PlayerFieldUnitDamageInfo {
        &self.player_field_unit_damage_info
    }

    pub fn get_player_field_unit_health_point_info(&self) -> &PlayerFieldUnitHealthPointInfo {
        &self.player_field_unit_health_point_info
    }

    pub fn get_player_field_unit_death_info(&self) -> &PlayerFieldUnitDeathInfo {
        &self.player_field_unit_death_info
    }
}