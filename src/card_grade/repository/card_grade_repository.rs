use async_trait::async_trait;

#[async_trait]
pub trait CardGradeRepository {
    async fn get_card_grade(&self, card_number: &i32) -> Option<i32>;

    async fn get_legend_mythical_card_list(&self, card_list: Vec<i32>) -> Vec<(i32)>;

}