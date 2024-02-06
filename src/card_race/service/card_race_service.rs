use async_trait::async_trait;

#[async_trait]
pub trait CardRaceService {
    async fn get_card_race(&self, card_number: &i32) -> Option<i32>;
}