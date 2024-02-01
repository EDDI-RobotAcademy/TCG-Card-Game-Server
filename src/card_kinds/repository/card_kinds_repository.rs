use async_trait::async_trait;

#[async_trait]
pub trait CardKindsRepository {
    async fn get_card_kind(&self, card_number: &str) -> Option<String>;
}