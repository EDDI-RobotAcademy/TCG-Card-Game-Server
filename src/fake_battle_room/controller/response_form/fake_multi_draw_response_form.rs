use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use crate::account::service::response::account_login_response::AccountLoginResponse;
use crate::game_card_support::controller::response_form::draw_support_response_form::DrawSupportResponseForm;
use crate::ui_data_generator::entity::player_index_enum::PlayerIndex;
use crate::ui_data_generator::service::response::generate_draw_my_deck_data_response::GenerateDrawMyDeckDataResponse;
use crate::ui_data_generator::service::response::generate_use_my_hand_card_data_response::GenerateUseMyHandCardDataResponse;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FakeMultiDrawResponseForm {
    player_multi_drawn_card_list: HashMap<PlayerIndex, Vec<i32>>
}

impl FakeMultiDrawResponseForm {
    pub fn new(player_multi_drawn_card_list: HashMap<PlayerIndex, Vec<i32>>) -> Self {

        Self { player_multi_drawn_card_list }
    }

    pub fn player_multi_drawn_card_list(&self) -> &HashMap<PlayerIndex, Vec<i32>> {
        &self.player_multi_drawn_card_list
    }

    pub fn from_response(
        generate_draw_my_deck_data_response: GenerateDrawMyDeckDataResponse
    ) -> FakeMultiDrawResponseForm {

        FakeMultiDrawResponseForm::new(
            generate_draw_my_deck_data_response
                .get_player_drawn_card_list_map_for_response().clone())
    }
}