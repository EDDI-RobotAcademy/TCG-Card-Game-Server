use std::collections::HashMap;
use async_trait::async_trait;
use diesel::result::Error;
use crate::account_card::entity::account_card_list::AccountCardList;


#[async_trait]
pub trait AccountCardRepository {
    async fn get_card_list(&self, request: i32) -> Result<Option<Vec<HashMap<i32, i32>>>, Error>;

    async fn check_same_card(&self, get_card_id: i32, account_card_list: Vec<HashMap<i32, i32>>) -> i32 ;
    async fn update_card_count(&self, shop_account_id: i32, shop_update_card: (i32, i32)) -> Result<usize, diesel::result::Error>;
    async fn save_new_card(&self, shop_account_id: i32, shop_card_id: i32) -> Result<(), diesel::result::Error>;
    async fn delete_all_account_cards(&self, account_unique_id: i32) -> Result<(), Error>;
    async fn get_account_card_list(&self, request_deck_id: i32) -> AccountCardList;
}