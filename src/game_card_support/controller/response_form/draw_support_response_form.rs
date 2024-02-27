use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use crate::notify_player_action_info::entity::player_index_enum::PlayerIndex;
use crate::notify_player_action_info::entity::player_index_enum::PlayerIndex::You;
use crate::notify_player_action_info::service::response::notice_draw_card_response::NoticeDrawCardResponse;
use crate::notify_player_action_info::service::response::notice_use_hand_card_response::NoticeUseHandCardResponse;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DrawSupportResponseForm {
    is_success: bool,
    player_draw_card_list_map: HashMap<PlayerIndex, Vec<i32>>
}

impl DrawSupportResponseForm {
    pub fn new(is_success: bool,
               player_draw_card_list_map: HashMap<PlayerIndex, Vec<i32>>) -> Self {
        DrawSupportResponseForm {
            is_success,
            player_draw_card_list_map
        }
    }

    pub fn from_response(notice_use_hand_card_response: NoticeUseHandCardResponse,
                         notice_draw_card_response: NoticeDrawCardResponse)
        -> DrawSupportResponseForm {

        DrawSupportResponseForm::new(
            notice_use_hand_card_response.is_success(),
            notice_draw_card_response
                .get_player_drawn_card_list_info()
                .get_player_drawn_card_list_map().clone())
    }

    pub fn default() -> DrawSupportResponseForm {

        DrawSupportResponseForm::new(
            false,
            HashMap::new())
    }
}