use serde::{Deserialize, Serialize};
use crate::ui_data_generator::entity::player_deck_card_lost_list_info::PlayerDeckCardLostListInfo;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NoticeLostDeckCardOfOpponentResponse {
    player_deck_card_lost_list_info: PlayerDeckCardLostListInfo,
}

impl NoticeLostDeckCardOfOpponentResponse {
    pub fn new(player_deck_card_lost_list_info: PlayerDeckCardLostListInfo) -> Self {
        NoticeLostDeckCardOfOpponentResponse {
            player_deck_card_lost_list_info
        }
    }

    pub fn get_player_deck_card_lost_list_info(&self) -> &PlayerDeckCardLostListInfo {
        &self.player_deck_card_lost_list_info
    }
}