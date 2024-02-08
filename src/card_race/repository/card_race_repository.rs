use async_trait::async_trait;
use crate::common::card_attributes::card_race::card_race_enum::RaceEnum;

#[async_trait]
pub trait CardRaceRepository {
    async fn get_card_race(&self, card_number: &i32) -> RaceEnum;

    async fn get_specific_race_card_list(&self , race_value: RaceEnum) -> Vec<i32>;

}