use async_trait::async_trait;

#[async_trait]
pub trait CardKindsService {
    async fn get_card_kind(&self, card_number: &i32) -> Option<i32>;
}