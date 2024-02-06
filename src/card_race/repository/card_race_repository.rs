use async_trait::async_trait;

#[async_trait]
pub trait CardRaceRepository {
    async fn get_card_race(&self, card_number: &i32) -> Option<i32>;

    async fn get_specific_race_card_list(&self , race_value : i32) -> Vec<i32> ;

}