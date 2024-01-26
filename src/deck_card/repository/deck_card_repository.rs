use std::collections::HashMap;
use async_trait::async_trait;
use diesel::result::Error;

use crate::deck_card::service::request::deck_configuration_request::DeckConfigurationRequest;

#[async_trait]
pub trait DeckCardRepository {
    async fn save(&self, deck_card: DeckConfigurationRequest) -> Result<String, String>;
    async fn get_card_list(&self, request: i32) -> Result<Option<Vec<HashMap<i32, i32>>>, Error>;
}