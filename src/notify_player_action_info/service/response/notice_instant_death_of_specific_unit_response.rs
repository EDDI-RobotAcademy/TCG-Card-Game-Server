use serde::{Deserialize, Serialize};
use crate::ui_data_generator::entity::player_field_unit_death_info::PlayerFieldUnitDeathInfo;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NoticeInstantDeathOfSpecificUnitResponse {
    player_field_unit_death_info: PlayerFieldUnitDeathInfo
}

impl NoticeInstantDeathOfSpecificUnitResponse {
    pub fn new(player_field_unit_death_info: PlayerFieldUnitDeathInfo) -> Self {
        NoticeInstantDeathOfSpecificUnitResponse {
            player_field_unit_death_info
        }
    }

    pub fn get_player_field_unit_death_info(&self) -> &PlayerFieldUnitDeathInfo { &self.player_field_unit_death_info }
}