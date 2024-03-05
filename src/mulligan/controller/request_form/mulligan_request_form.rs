use crate::battle_room::service::request::find_opponent_by_account_id_request::FindOpponentByAccountIdRequest;
use crate::common::converter::vector_string_to_vector_integer::VectorStringToVectorInteger;
use crate::game_deck::service::request::draw_cards_from_deck_request::DrawCardsFromDeckRequest;
use crate::game_deck::service::request::game_deck_card_list_request::GameDeckCardListRequest;
use crate::game_deck::service::request::game_deck_card_shuffle_request::GameDeckCardShuffleRequest;
use crate::game_field_energy::service::request::add_field_energy_with_amount_request::AddFieldEnergyWithAmountRequest;
use crate::game_hand::service::request::add_card_list_to_hand_request::AddCardListToHandRequest;
use crate::game_hand::service::request::put_cards_on_deck_request::PutCardsOnDeckRequest;
use crate::game_protocol_validation::service::request::check_cards_from_hand_request::CheckCardsFromHandRequest;
use crate::game_turn::service::request::next_turn_request::NextTurnRequest;
use crate::redis::service::request::get_value_with_key_request::GetValueWithKeyRequest;
use crate::rock_paper_scissors::service::request::check_rock_paper_scissors_winner_request::CheckRockPaperScissorsWinnerRequest;
use crate::ui_data_generator::service::request::generate_draw_my_deck_data_request::GenerateDrawMyDeckDataRequest;
use crate::ui_data_generator::service::request::generate_my_field_energy_data_request::GenerateMyFieldEnergyDataRequest;


#[derive(Debug)]
pub struct MulliganRequestForm {
    session_id: String,
    will_be_changed_card_list: Vec<String>,
}

impl MulliganRequestForm {
    pub fn new(
        session_id: String,
        will_be_changed_card_list: Vec<String>
    ) -> Self {

        MulliganRequestForm {
            session_id,
            will_be_changed_card_list
        }
    }

    pub fn to_put_cards_on_deck_request(
        &self) -> PutCardsOnDeckRequest {

        PutCardsOnDeckRequest::new(
            self.session_id.clone(),
            self.will_be_changed_card_list.clone()
        )
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

    pub fn to_check_cards_from_hand_request(
        &self) -> CheckCardsFromHandRequest {

        let maybe_hand_card_list =
            VectorStringToVectorInteger::vector_string_to_vector_i32(self.will_be_changed_card_list.clone());

        CheckCardsFromHandRequest::new(&self.session_id, maybe_hand_card_list)
    }

    pub fn to_shuffle_deck_request(&self) -> GameDeckCardShuffleRequest {
        GameDeckCardShuffleRequest::new(self.session_id.clone())
    }

    pub fn to_check_rock_paper_scissors_winner_request(
        &self,
        account_unique_id: i32,
        opponent_unique_id: i32
    ) -> CheckRockPaperScissorsWinnerRequest {

        CheckRockPaperScissorsWinnerRequest::new(
            account_unique_id, opponent_unique_id)
    }

    pub fn to_draw_cards_from_deck_request(
        &self,
        account_unique_id: i32) -> DrawCardsFromDeckRequest {

        DrawCardsFromDeckRequest::new(
            account_unique_id, self.will_be_changed_card_list.clone().len() as i32)
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

    pub fn to_get_deck_request(&self) -> GameDeckCardListRequest {
        GameDeckCardListRequest::new(self.session_id.clone())
    }

    pub fn to_next_turn_request(
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
        &self,
        remaining_field_energy: i32) -> GenerateMyFieldEnergyDataRequest {

        GenerateMyFieldEnergyDataRequest::new(
            remaining_field_energy)
    }
}