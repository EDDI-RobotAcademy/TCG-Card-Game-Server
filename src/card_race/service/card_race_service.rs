use async_trait::async_trait;
use crate::common::card_attributes::card_race::card_race_enum::RaceEnum;

#[async_trait]
pub trait CardRaceService {
    async fn get_card_race(&self, card_number: &i32) -> RaceEnum;
}