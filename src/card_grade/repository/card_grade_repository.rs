use async_trait::async_trait;

#[async_trait]
pub trait CardGradeRepository {
    async fn get_card_grade(&self, card_number: &i32) -> Option<String>;

    async fn get_grade_by_specific_race_card_list(&self, race_specific_card_list: Vec<i32>) -> Vec<(i32,String)>;

}