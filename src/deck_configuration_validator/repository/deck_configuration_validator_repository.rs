use async_trait::async_trait;

#[async_trait]
pub trait DeckConfigurationValidatorRepository {
    async fn counter(&self, deck: &Vec<i32>) -> Result<(), String>;
    async fn duplication_checker(&self, deck: &Vec<i32>) -> Result<(), String>;
}