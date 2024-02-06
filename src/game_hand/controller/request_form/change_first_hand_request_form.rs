use crate::game_deck::service::request::game_deck_card_redraw_request::GameDeckCardRedrawRequest;
use crate::game_hand::service::request::put_cards_on_deck_request::PutCardsOnDeckRequest;


#[derive(Debug)]
pub struct ChangeFirstHandRequestForm {
    session_id: String,
    will_be_changed_card_list: Vec<String>,
}

impl ChangeFirstHandRequestForm {
    pub fn new(session_id: String, will_be_changed_card_list: Vec<String>) -> Self {
        ChangeFirstHandRequestForm {
            session_id,
            will_be_changed_card_list
        }
    }
    pub fn to_put_cards_on_deck_request(&self) -> PutCardsOnDeckRequest {
        PutCardsOnDeckRequest::new(
            self.session_id.clone(),
            self.will_be_changed_card_list.clone()
        )
    }
    pub fn to_shuffle_and_redraw_card_request(&self) -> GameDeckCardRedrawRequest {
        GameDeckCardRedrawRequest::new(
            self.session_id.clone(),
            self.will_be_changed_card_list.clone().len() as i32
        )
    }
}