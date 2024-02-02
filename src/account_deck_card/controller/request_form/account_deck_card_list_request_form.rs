use crate::account_deck_card::service::request::account_deck_card_list_request::AccountDeckCardListRequest;

#[derive(Debug)]
pub struct AccountDeckCardListRequestFrom {
    deck_id_form: i32
}

impl AccountDeckCardListRequestFrom {
    pub fn new(deck_id_form: i32) -> Self { AccountDeckCardListRequestFrom { deck_id_form } }
    pub fn to_account_deck_card_list_request(&self) -> AccountDeckCardListRequest {
        AccountDeckCardListRequest::new( self.deck_id_form )
    }
}