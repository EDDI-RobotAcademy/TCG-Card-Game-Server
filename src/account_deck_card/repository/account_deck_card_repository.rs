use std::collections::HashMap;
use async_trait::async_trait;
use diesel::result::Error;

use crate::account_deck_card::entity::account_deck_card::AccountDeckCard;
use crate::account_deck_card::entity::account_deck_card_list::AccountDeckCardList;

#[async_trait]
pub trait AccountDeckCardRepository {
    async fn save_deck_card_list(&self, deck_card_list: Vec<AccountDeckCard>) -> Result<String, String>;
    async fn get_card_list(&self, request_deck_id: i32) -> Result<Option<Vec<HashMap<i32, i32>>>, Error>;
    async fn get_account_deck_card_list(&self, request_deck_id: i32) -> AccountDeckCardList;
    async fn delete_deck_cards(&self, deck_unique_id: i32) -> Result<(), Error>;
}