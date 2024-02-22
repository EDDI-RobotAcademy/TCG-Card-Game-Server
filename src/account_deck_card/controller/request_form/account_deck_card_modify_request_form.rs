use crate::account_deck_card::service::request::account_deck_configuration_request::AccountDeckConfigurationRequest;

#[derive(Debug)]
pub struct AccountDeckCardModifyRequestForm {
    deck_id_form: i32,
    card_id_list_form: Vec<i32>
}

impl AccountDeckCardModifyRequestForm {
    pub fn new(deck_id_form: i32, card_id_list_form: Vec<i32>) -> Self {
        AccountDeckCardModifyRequestForm {
            deck_id_form,
            card_id_list_form
        }
    }
    pub fn deck_id_form (&self) -> i32 { self.deck_id_form }
    pub fn card_id_list_form(&self) -> Vec<i32> { self.card_id_list_form.clone() }
    pub fn to_account_deck_configuration_request(&self) -> AccountDeckConfigurationRequest {
        AccountDeckConfigurationRequest::new( self.deck_id_form, self.card_id_list_form.clone() )
    }
}