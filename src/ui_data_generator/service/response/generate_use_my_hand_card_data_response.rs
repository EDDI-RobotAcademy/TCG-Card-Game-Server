use std::collections::HashMap;
use crate::ui_data_generator::entity::player_index_enum::PlayerIndex;
use crate::ui_data_generator::entity::used_hand_card_info::UsedHandCardInfo;

#[derive(Debug)]
pub struct GenerateUseMyHandCardDataResponse {
    is_success: bool,
    player_hand_use_map_for_notice: HashMap<PlayerIndex, UsedHandCardInfo>,
}

impl GenerateUseMyHandCardDataResponse {
    pub fn new(is_success: bool,
               player_hand_use_map_for_notice: HashMap<PlayerIndex, UsedHandCardInfo>,) -> Self {
        GenerateUseMyHandCardDataResponse {
            is_success,
            player_hand_use_map_for_notice
        }
    }

    pub fn is_success(&self) -> bool { self.is_success }

    pub fn get_player_hand_use_map_for_notice(&self) -> &HashMap<PlayerIndex, UsedHandCardInfo> {
        &self.player_hand_use_map_for_notice
    }
}