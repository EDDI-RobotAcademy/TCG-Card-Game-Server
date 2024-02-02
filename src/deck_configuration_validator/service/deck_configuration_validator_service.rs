use async_trait::async_trait;

#[async_trait]
pub trait DeckConfigurationValidatorService {
    async fn validate_deck(&self, deck: &Vec<i32>) -> Result<(), String>;
}