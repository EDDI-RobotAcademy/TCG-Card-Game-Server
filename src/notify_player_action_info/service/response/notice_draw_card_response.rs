use serde::{Deserialize, Serialize};
use crate::notify_player_action_info::entity::player_drawn_card_list_info::PlayerDrawnCardListInfo;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NoticeDrawCardResponse {
    player_drawn_card_list_info: PlayerDrawnCardListInfo,
}

impl NoticeDrawCardResponse {
    pub fn new(player_drawn_card_list_info: PlayerDrawnCardListInfo) -> Self {
        NoticeDrawCardResponse {
            player_drawn_card_list_info
        }
    }

    pub fn get_player_drawn_card_list_info(&self) -> &PlayerDrawnCardListInfo {
        &self.player_drawn_card_list_info
    }
}