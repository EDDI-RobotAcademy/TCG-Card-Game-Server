use crate::game_deck::service::request::found_card_from_deck_request::FoundCardFromDeckRequest;
use crate::game_deck::service::request::search_specific_deck_card_request::SearchSpecificDeckCardRequest;
use crate::game_hand::service::request::add_card_list_to_hand_request::AddCardListToHandRequest;
use crate::redis::service::request::get_value_with_key_request::GetValueWithKeyRequest;
use crate::ui_data_generator::service::request::generate_search_my_deck_data_request::GenerateSearchMyDeckDataRequest;

#[derive(Debug)]
pub struct FakeGetNetherFromDeckRequestForm {
    session_info: String,
}

impl FakeGetNetherFromDeckRequestForm {
    pub fn new(
        session_info: String) -> Self {

        FakeGetNetherFromDeckRequestForm {
            session_info: session_info.to_string(),
        }
    }

    pub fn to_session_validation_request(
        &self) -> GetValueWithKeyRequest {

        GetValueWithKeyRequest::new(
            self.session_info.clone().as_str())
    }

    pub fn to_find_card_from_deck_with_count_request(
        &self,
        account_unique_id: i32,
        need_to_find_card_id: i32,
        card_count: i32) -> FoundCardFromDeckRequest {

        FoundCardFromDeckRequest::new(
            account_unique_id,
            need_to_find_card_id,
            card_count)
    }

    pub fn to_add_card_list_to_hand_request(
        &self,
        account_unique_id: i32,
        card_list: Vec<i32>) -> AddCardListToHandRequest {

        AddCardListToHandRequest::new(
            account_unique_id,
            card_list)
    }

    pub fn to_generate_search_my_deck_data_request(
        &self,
        found_card_list: Vec<i32>
    ) -> GenerateSearchMyDeckDataRequest {

        GenerateSearchMyDeckDataRequest::new(
            found_card_list)
    }
}
