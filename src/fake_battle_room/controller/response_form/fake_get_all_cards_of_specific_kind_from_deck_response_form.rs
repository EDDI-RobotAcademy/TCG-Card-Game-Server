use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use crate::ui_data_generator::entity::player_index_enum::PlayerIndex;
use crate::ui_data_generator::service::response::generate_search_my_deck_data_response::GenerateSearchMyDeckDataResponse;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FakeGetAllCardsOfSpecificKindFromDeckResponseForm {
    player_search_card_list_map: HashMap<PlayerIndex, Vec<i32>>,
}

impl FakeGetAllCardsOfSpecificKindFromDeckResponseForm {
    pub fn new(player_search_card_list_map: HashMap<PlayerIndex, Vec<i32>>) -> Self {
        FakeGetAllCardsOfSpecificKindFromDeckResponseForm {
            player_search_card_list_map
        }
    }

    pub fn from_response(
        generate_search_my_deck_data_response: GenerateSearchMyDeckDataResponse
    ) -> FakeGetAllCardsOfSpecificKindFromDeckResponseForm {

        FakeGetAllCardsOfSpecificKindFromDeckResponseForm::new(
            generate_search_my_deck_data_response
                .get_player_search_card_list_map_for_response().clone())
    }

    pub fn default() -> FakeGetAllCardsOfSpecificKindFromDeckResponseForm {
        FakeGetAllCardsOfSpecificKindFromDeckResponseForm::new(HashMap::new())
    }
}