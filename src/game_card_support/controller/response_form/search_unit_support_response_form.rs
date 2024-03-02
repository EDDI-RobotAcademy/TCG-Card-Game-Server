use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use crate::ui_data_generator::entity::player_index_enum::PlayerIndex;
use crate::ui_data_generator::service::response::generate_search_my_deck_data_response::GenerateSearchMyDeckDataResponse;
use crate::ui_data_generator::service::response::generate_use_my_hand_card_data_response::GenerateUseMyHandCardDataResponse;

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

    pub fn from_response(generate_use_my_hand_card_data_response: GenerateUseMyHandCardDataResponse,
                         generate_search_my_deck_data_response: GenerateSearchMyDeckDataResponse)
        -> SearchUnitSupportResponseForm {

        SearchUnitSupportResponseForm::new(
            generate_use_my_hand_card_data_response.is_success_for_response(),
            generate_search_my_deck_data_response
                .get_player_search_card_list_map_for_response().clone())
    }

    pub fn default() -> SearchUnitSupportResponseForm {

        SearchUnitSupportResponseForm::new(
            false,
            HashMap::new())
    }
}