use async_trait::async_trait;
use diesel::result::Error;

#[async_trait]
pub trait ShopRepository {
    async fn add_free_cards(&self, account_id: i32) -> Result<Vec<i32>, Error>;
    fn get_randomly_chosen_card_id_list(how_many_cards_to_get: i32) -> Result<Vec<i32>, Error>;
}