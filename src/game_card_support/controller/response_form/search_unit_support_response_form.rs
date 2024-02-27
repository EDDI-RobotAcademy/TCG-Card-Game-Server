use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use crate::notify_player_action_info::entity::player_index_enum::PlayerIndex;
use crate::notify_player_action_info::entity::player_index_enum::PlayerIndex::You;
use crate::notify_player_action_info::service::response::notice_search_card_response::NoticeSearchCardResponse;
use crate::notify_player_action_info::service::response::notice_use_hand_card_response::NoticeUseHandCardResponse;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchUnitSupportResponseForm {
    is_success: bool,
    player_search_card_list_map: HashMap<PlayerIndex, Vec<i32>>,
}

impl SearchUnitSupportResponseForm {
    pub fn new(is_success: bool,
               player_search_card_list_map: HashMap<PlayerIndex, Vec<i32>>,) -> Self {
        SearchUnitSupportResponseForm {
            is_success,
            player_search_card_list_map
        }
    }

    pub fn from_response(notice_use_hand_card_response: NoticeUseHandCardResponse,
                         notice_search_card_response: NoticeSearchCardResponse)
        -> SearchUnitSupportResponseForm {

        SearchUnitSupportResponseForm::new(
            notice_use_hand_card_response.is_success(),
            notice_search_card_response
                .get_player_search_card_list_info()
                .get_player_search_card_list_map().clone())
    }

    pub fn default() -> SearchUnitSupportResponseForm {

        SearchUnitSupportResponseForm::new(
            false,
            HashMap::new())
    }
}