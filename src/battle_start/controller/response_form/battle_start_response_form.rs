use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use crate::ui_data_generator::entity::player_index_enum::PlayerIndex;
use crate::ui_data_generator::service::response::generate_draw_my_deck_data_response::GenerateDrawMyDeckDataResponse;
use crate::ui_data_generator::service::response::generate_draw_opponent_deck_data_response::GenerateDrawOpponentDeckDataResponse;
use crate::ui_data_generator::service::response::generate_my_field_energy_data_response::GenerateMyFieldEnergyDataResponse;
use crate::ui_data_generator::service::response::generate_opponent_field_energy_data_response::GenerateOpponentFieldEnergyDataResponse;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BattleStartResponseForm {
    is_your_turn: bool,
    player_draw_card_list_map: HashMap<PlayerIndex, Vec<i32>>,
    player_draw_count_map: HashMap<PlayerIndex, i32>,
    player_field_energy_map: HashMap<PlayerIndex, i32>,
}

impl BattleStartResponseForm {
    pub fn new(is_your_turn: bool,
               player_draw_card_list_map: HashMap<PlayerIndex, Vec<i32>>,
               player_draw_count_map: HashMap<PlayerIndex, i32>,
               player_field_energy_map: HashMap<PlayerIndex, i32>
    ) -> Self {

        BattleStartResponseForm {
            is_your_turn,
            player_draw_card_list_map,
            player_draw_count_map,
            player_field_energy_map,
        }
    }

    pub fn from_response_for_winner(
        generate_draw_my_deck_data_response: GenerateDrawMyDeckDataResponse,
        generate_my_field_energy_data_response: GenerateMyFieldEnergyDataResponse,
    ) -> BattleStartResponseForm {

        BattleStartResponseForm::new(
            true,
            generate_draw_my_deck_data_response
                .get_player_drawn_card_list_map_for_response().clone(),
            HashMap::new(),
            generate_my_field_energy_data_response
                .get_player_field_energy_map_for_response().clone())
    }

    pub fn from_response_for_loser(
        generate_draw_opponent_deck_data_response: GenerateDrawOpponentDeckDataResponse,
        generate_opponent_field_energy_data_response: GenerateOpponentFieldEnergyDataResponse,
    ) -> BattleStartResponseForm {

        BattleStartResponseForm::new(
            false,
            HashMap::new(),
            generate_draw_opponent_deck_data_response
                .get_player_drawn_card_count_map_for_response().clone(),
            generate_opponent_field_energy_data_response
                .get_player_field_energy_map_for_response().clone())
    }

    pub fn default() -> BattleStartResponseForm {
        BattleStartResponseForm::new(
            false,
            HashMap::new(),
            HashMap::new(),
            HashMap::new())
    }
}