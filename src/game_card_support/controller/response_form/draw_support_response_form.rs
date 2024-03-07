use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use crate::game_deck::service::response::game_deck_card_list_response::GameDeckCardListResponse;
use crate::ui_data_generator::entity::player_index_enum::PlayerIndex;
use crate::ui_data_generator::service::response::generate_draw_my_deck_data_response::GenerateDrawMyDeckDataResponse;
use crate::ui_data_generator::service::response::generate_use_my_hand_card_data_response::GenerateUseMyHandCardDataResponse;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DrawSupportResponseForm {
    is_success: bool,
    player_draw_card_list_map: HashMap<PlayerIndex, Vec<i32>>,
    updated_deck_card_list: Vec<i32>,
}

impl DrawSupportResponseForm {
    pub fn new(is_success: bool,
               player_draw_card_list_map: HashMap<PlayerIndex, Vec<i32>>,
               updated_deck_card_list: Vec<i32>,) -> Self {
        DrawSupportResponseForm {
            is_success,
            player_draw_card_list_map,
            updated_deck_card_list
        }
    }

    pub fn from_response(generate_use_hand_card_data_response: GenerateUseMyHandCardDataResponse,
                         generate_draw_my_deck_data_response: GenerateDrawMyDeckDataResponse,
                         game_deck_card_list_response: GameDeckCardListResponse
    ) -> DrawSupportResponseForm {

        DrawSupportResponseForm::new(
            generate_use_hand_card_data_response.is_success_for_response(),
            generate_draw_my_deck_data_response
                .get_player_drawn_card_list_map_for_response().clone(),
            game_deck_card_list_response.get_deck_card_list().clone())
    }

    pub fn default() -> DrawSupportResponseForm {

        DrawSupportResponseForm::new(
            false,
            HashMap::new(),
            Vec::new())
    }
}