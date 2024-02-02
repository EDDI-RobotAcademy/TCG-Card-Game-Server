use async_trait::async_trait;

#[async_trait]
pub trait CardRaceRepository {
    async fn get_card_race(&self, card_number: &i32) -> Option<String>;
}