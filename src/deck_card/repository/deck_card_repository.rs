use async_trait::async_trait;

use crate::deck_card::service::request::deck_configuration_request::DeckConfigurationRequest;

#[async_trait]
pub trait DeckCardRepository {
    async fn save(&self, deck_card: DeckConfigurationRequest) -> Result<String, String>;
}