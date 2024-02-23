use std::collections::HashMap;
use std::hash::Hash;
use async_trait::async_trait;
use diesel::result::Error;
use crate::account_deck::entity::account_deck::AccountDeck;
use crate::account_deck::service::request::account_deck_modify_request::AccountDeckModifyRequest;

#[async_trait]
pub trait AccountDeckRepository {
    async fn save(&self, deck: AccountDeck) -> Result<(), Error>;
    async fn get_account_deck_id_list(&self, account_unique_id: i32) -> Result<Vec<i32>, Error>;
    async fn get_list_by_user_int_id(&self, request: i32) -> Result<Option<Vec<HashMap<i32, String>>>, Error>;
    async fn update(&self, modify_deck: AccountDeckModifyRequest, int_id: i32) -> Result<(), Error>;
    async fn delete(&self, deck_id: i32) -> Result<(), Error>;
    async fn delete_all_account_decks(&self, account_unique_id: i32) -> Result<(), Error>;
    async fn deck_owner_verification(&self, account_deck_list: Vec<HashMap<i32, String>>, deck_id: i32) -> bool ;
}