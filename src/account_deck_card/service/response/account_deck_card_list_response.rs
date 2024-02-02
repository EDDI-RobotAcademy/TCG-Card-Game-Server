use std::collections::HashMap;
use serde::{Deserialize, Serialize};

use crate::account_deck_card::controller::response_form::account_deck_card_list_response_form::AccountDeckCardListResponseForm;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccountDeckCardListResponse {
    card_id_list: Vec<HashMap<i32, i32>>
}

impl AccountDeckCardListResponse {
    pub fn new(card_id_list: Vec<HashMap<i32, i32>>) -> Self { AccountDeckCardListResponse { card_id_list } }
    pub fn to_account_deck_card_list_response_form(&self) -> AccountDeckCardListResponseForm {
        AccountDeckCardListResponseForm::new( self.card_id_list.clone() )
    }
}