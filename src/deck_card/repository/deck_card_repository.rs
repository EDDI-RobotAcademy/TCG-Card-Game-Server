use std::collections::HashMap;
use async_trait::async_trait;
use diesel::result::Error;
use crate::deck_card::entity::deck_card::DeckCard;

#[async_trait]
pub trait DeckCardRepository {
    async fn save_deck_card_list(&self, deck_card_list: Vec<DeckCard>) -> Result<String, String>;
    async fn get_card_list(&self, request_deck_id: i32) -> Result<Option<Vec<HashMap<i32, i32>>>, Error>;
}