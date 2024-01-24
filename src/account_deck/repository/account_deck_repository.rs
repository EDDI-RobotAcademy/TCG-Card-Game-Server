use async_trait::async_trait;
use crate::account_deck::entity::account_deck::AccountDeck;

#[async_trait]
pub trait AccountDeckRepository {
    async fn save(&self, deck: AccountDeck) -> Result<(), diesel::result::Error>;
}