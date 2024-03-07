use crate::account::service::request::account_login_request::AccountLoginRequest;
use crate::fake_battle_room::service::request::create_battle_fake_room_request::CreateFakeBattleRoomRequest;
use crate::game_deck::service::request::draw_cards_from_deck_request::DrawCardsFromDeckRequest;
use crate::game_hand::service::request::add_card_list_to_hand_request::AddCardListToHandRequest;
use crate::redis::service::request::get_value_with_key_request::GetValueWithKeyRequest;
use crate::ui_data_generator::service::request::generate_draw_my_deck_data_request::GenerateDrawMyDeckDataRequest;

#[derive(Debug)]
pub struct FakeMultiDrawRequestForm {
    session_info: String,
}

impl FakeMultiDrawRequestForm {
    pub fn new(
        session_info: String) -> Self {

        FakeMultiDrawRequestForm {
            session_info: session_info.to_string(),
        }
    }

    pub fn get_session_info(&self) -> &str {
        &self.session_info
    }

    pub fn to_session_validation_request(
        &self) -> GetValueWithKeyRequest {

        GetValueWithKeyRequest::new(
            self.session_info.clone().as_str())
    }

    pub fn to_draw_cards_from_deck_request(
        &self,
        account_unique_id: i32,
        draw_count: i32) -> DrawCardsFromDeckRequest {

        DrawCardsFromDeckRequest::new(
            account_unique_id,
            draw_count)
    }

    pub fn to_add_card_list_to_hand_request(
        &self,
        account_unique_id: i32,
        card_list: Vec<i32>) -> AddCardListToHandRequest {

        AddCardListToHandRequest::new(
            account_unique_id,
            card_list)
    }

    pub fn to_generate_draw_my_deck_data_request(
        &self,
        drawn_card_list: Vec<i32>
    ) -> GenerateDrawMyDeckDataRequest {

        GenerateDrawMyDeckDataRequest::new(
            drawn_card_list)
    }
}
