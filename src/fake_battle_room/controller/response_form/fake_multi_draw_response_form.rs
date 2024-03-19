use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use crate::account::service::response::account_login_response::AccountLoginResponse;
use crate::game_card_support::controller::response_form::draw_support_response_form::DrawSupportResponseForm;
use crate::game_deck::service::response::game_deck_card_list_response::GameDeckCardListResponse;
use crate::ui_data_generator::entity::player_index_enum::PlayerIndex;
use crate::ui_data_generator::service::response::generate_draw_my_deck_data_response::GenerateDrawMyDeckDataResponse;
use crate::ui_data_generator::service::response::generate_use_my_hand_card_data_response::GenerateUseMyHandCardDataResponse;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FakeMultiDrawResponseForm {
    player_multi_drawn_card_list: HashMap<PlayerIndex, Vec<i32>>,
    updated_deck_card_list: Vec<i32>
}

impl FakeMultiDrawResponseForm {
    pub fn new(player_multi_drawn_card_list: HashMap<PlayerIndex, Vec<i32>>,
               updated_deck_card_list: Vec<i32>) -> Self {

        Self { player_multi_drawn_card_list, updated_deck_card_list }
    }

    pub fn player_multi_drawn_card_list(&self) -> &HashMap<PlayerIndex, Vec<i32>> {
        &self.player_multi_drawn_card_list
    }

    pub fn from_response(
        generate_draw_my_deck_data_response: GenerateDrawMyDeckDataResponse,
        get_deck_response: GameDeckCardListResponse,
    ) -> FakeMultiDrawResponseForm {

        FakeMultiDrawResponseForm::new(
            generate_draw_my_deck_data_response
                .get_player_drawn_card_list_map_for_response().clone(),
            get_deck_response.get_deck_card_list().clone())
    }

    pub fn default() -> FakeMultiDrawResponseForm {

        FakeMultiDrawResponseForm::new(HashMap::new(), Vec::new())
    }
}