use std::collections::HashMap;
use async_trait::async_trait;
use diesel::result::Error;

#[async_trait]
pub trait ShopRepository {
    // async fn add_free_cards(&self, account_id: i32) -> Result<Vec<i32>, Error>;
    async fn get_randomly_chosen_card_id_list(&self, how_many_cards_to_get: i32) -> Result<Vec<i32>, Error> ;

}