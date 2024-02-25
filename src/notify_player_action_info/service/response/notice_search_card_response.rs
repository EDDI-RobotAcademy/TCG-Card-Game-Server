use serde::{Deserialize, Serialize};
use crate::notify_player_action_info::entity::player_search_card_list_info::PlayerSearchCardListInfo;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NoticeSearchCardResponse {
    player_search_card_list_info: PlayerSearchCardListInfo,
}

impl NoticeSearchCardResponse {
    pub fn new(player_search_card_list_info: PlayerSearchCardListInfo) -> Self {
        NoticeSearchCardResponse {
            player_search_card_list_info
        }
    }

    pub fn get_player_search_card_list_info(&self) -> &PlayerSearchCardListInfo {
        &self.player_search_card_list_info
    }
}