use std::collections::HashMap;
use crate::ui_data_generator::entity::player_index_enum::PlayerIndex;
use crate::ui_data_generator::entity::used_hand_card_info::UsedHandCardInfo;

#[derive(Debug)]
pub struct NoticeUseDrawSupportCardRequest {
    opponent_unique_id: i32,
    player_hand_use_map_for_notice: HashMap<PlayerIndex, UsedHandCardInfo>,
    player_draw_count_map_for_notice: HashMap<PlayerIndex, i32>,
}

impl NoticeUseDrawSupportCardRequest {
    pub fn new(opponent_unique_id: i32,
               player_hand_use_map_for_notice: HashMap<PlayerIndex, UsedHandCardInfo>,
               player_draw_count_map_for_notice: HashMap<PlayerIndex, i32>,) -> Self {
        NoticeUseDrawSupportCardRequest {
            opponent_unique_id,
            player_hand_use_map_for_notice,
            player_draw_count_map_for_notice,
        }
    }

    pub fn get_opponent_unique_id(&self) -> i32 { self.opponent_unique_id }

    pub fn get_player_hand_use_map_for_notice(&self) -> &HashMap<PlayerIndex, UsedHandCardInfo> {
        &self.player_hand_use_map_for_notice
    }

    pub fn get_player_draw_count_map_for_notice(&self) -> &HashMap<PlayerIndex, i32> {
        &self.player_draw_count_map_for_notice
    }
}