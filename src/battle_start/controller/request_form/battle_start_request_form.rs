use crate::battle_room::service::request::find_opponent_by_account_id_request::FindOpponentByAccountIdRequest;
use crate::game_deck::service::request::draw_cards_from_deck_request::DrawCardsFromDeckRequest;
use crate::game_field_energy::service::request::add_field_energy_with_amount_request::AddFieldEnergyWithAmountRequest;
use crate::game_hand::service::request::add_card_list_to_hand_request::AddCardListToHandRequest;
use crate::game_turn::service::request::next_turn_request::NextTurnRequest;
use crate::redis::service::request::get_value_with_key_request::GetValueWithKeyRequest;
use crate::rock_paper_scissors::service::request::check_rock_paper_scissors_winner_request::CheckRockPaperScissorsWinnerRequest;
use crate::ui_data_generator::service::request::generate_draw_my_deck_data_request::GenerateDrawMyDeckDataRequest;
use crate::ui_data_generator::service::request::generate_draw_opponent_deck_data_request::GenerateDrawOpponentDeckDataRequest;
use crate::ui_data_generator::service::request::generate_my_field_energy_data_request::GenerateMyFieldEnergyDataRequest;
use crate::ui_data_generator::service::request::generate_opponent_field_energy_data_request::GenerateOpponentFieldEnergyDataRequest;

#[derive(Debug)]
pub struct BattleStartRequestForm {
    session_id: String,
}

impl BattleStartRequestForm {
    pub fn new(
        session_id: String,
    ) -> Self {

        BattleStartRequestForm {
            session_id,
        }
    }

    pub fn to_get_value_with_key_request(
        &self) -> GetValueWithKeyRequest {

        GetValueWithKeyRequest::new(&self.session_id)
    }

    pub fn to_find_opponent_by_account_id_request(
        &self,
        account_unique_id: i32) -> FindOpponentByAccountIdRequest {

        FindOpponentByAccountIdRequest::new(
            account_unique_id)
    }

    pub fn to_check_rock_paper_scissors_winner_request(
        &self,
        account_unique_id: i32,
        opponent_unique_id: i32
    ) -> CheckRockPaperScissorsWinnerRequest {

        CheckRockPaperScissorsWinnerRequest::new(
            account_unique_id, opponent_unique_id)
    }

    pub fn to_next_turn_request(
        &self,
        account_unique_id: i32) -> NextTurnRequest {

        NextTurnRequest::new(
            account_unique_id)
    }

    pub fn to_draw_cards_from_deck_request(
        &self,
        account_unique_id: i32) -> DrawCardsFromDeckRequest {

        DrawCardsFromDeckRequest::new(
            account_unique_id, 1)
    }

    pub fn to_add_field_energy_request(
        &self,
        account_unique_id: i32) -> AddFieldEnergyWithAmountRequest {

        AddFieldEnergyWithAmountRequest::new(
            account_unique_id, 1)
    }

    pub fn to_add_card_list_to_hand_request(
        &self,
        account_unique_id: i32,
        card_list: Vec<i32>) -> AddCardListToHandRequest {

        AddCardListToHandRequest::new(
            account_unique_id, card_list)
    }

    pub fn to_next_turn_for_winner_request(
        &self,
        account_unique_id: i32) -> NextTurnRequest {

        NextTurnRequest::new(
            account_unique_id)
    }

    pub fn to_generate_draw_my_deck_data_request(
        &self,
        drawn_card_list: Vec<i32>,) -> GenerateDrawMyDeckDataRequest {

        GenerateDrawMyDeckDataRequest::new(
            drawn_card_list)
    }

    pub fn to_generate_my_field_energy_data_request(
        &self) -> GenerateMyFieldEnergyDataRequest {

        GenerateMyFieldEnergyDataRequest::new(
            1)
    }

    pub fn to_generate_draw_opponent_deck_data_request(
        &self) -> GenerateDrawOpponentDeckDataRequest {

        GenerateDrawOpponentDeckDataRequest::new(
            vec![-1])
    }

    pub fn to_generate_opponent_field_energy_data_request(
        &self) -> GenerateOpponentFieldEnergyDataRequest {

        GenerateOpponentFieldEnergyDataRequest::new(
            1)
    }
}