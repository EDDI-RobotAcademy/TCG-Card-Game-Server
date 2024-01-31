use std::collections::HashMap;
use async_trait::async_trait;
use diesel::result::Error;



#[async_trait]
pub trait AccountCardRepository {
    async fn get_card_list(&self, request: i32) -> Result<Option<Vec<HashMap<i32, i32>>>, Error>;
}