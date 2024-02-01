use async_trait::async_trait;

#[async_trait]
pub trait CardGradeService {
    async fn get_card_grade(&self, card_number: &i32) -> Option<String>;
}