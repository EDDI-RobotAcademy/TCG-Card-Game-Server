use serde::{Deserialize, Serialize};
use crate::notify_player_action_info::entity::player_main_character_damage_info::PlayerMainCharacterDamageInfo;
use crate::notify_player_action_info::entity::player_main_character_health_point_info::PlayerMainCharacterHealthPointInfo;
use crate::notify_player_action_info::entity::player_main_character_survival_info::PlayerMainCharacterSurvivalInfo;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NoticeApplyDamageToOpponentMainCharacterResponse {
    player_main_character_damage_info: PlayerMainCharacterDamageInfo,
    player_main_character_health_point_info: PlayerMainCharacterHealthPointInfo,
    player_main_character_survival_info: PlayerMainCharacterSurvivalInfo
}

impl NoticeApplyDamageToOpponentMainCharacterResponse {
    pub fn new(player_main_character_damage_info: PlayerMainCharacterDamageInfo,
               player_main_character_health_point_info: PlayerMainCharacterHealthPointInfo,
               player_main_character_survival_info: PlayerMainCharacterSurvivalInfo) -> Self {
        NoticeApplyDamageToOpponentMainCharacterResponse {
            player_main_character_damage_info,
            player_main_character_health_point_info,
            player_main_character_survival_info
        }
    }

    pub fn get_player_main_character_damage_info(&self) -> &PlayerMainCharacterDamageInfo {
        &self.player_main_character_damage_info
    }

    pub fn get_player_main_character_health_point_info(&self) -> &PlayerMainCharacterHealthPointInfo {
        &self.player_main_character_health_point_info
    }

    pub fn get_player_main_character_survival_info(&self) -> &PlayerMainCharacterSurvivalInfo {
        &self.player_main_character_survival_info
    }
}