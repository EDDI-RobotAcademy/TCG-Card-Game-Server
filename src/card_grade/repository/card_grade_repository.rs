use async_trait::async_trait;

#[async_trait]
pub trait CardGradeRepository {
    async fn get_card_grade(&self, card_number: &i32) -> Option<i32>;

    async fn get_grade_by_card_list(&self, card_list: Vec<i32>) -> Vec<(i32,i32)>;

}