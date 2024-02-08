use async_trait::async_trait;
use crate::common::card_attributes::card_grade::card_grade_enum::GradeEnum;

#[async_trait]
pub trait CardGradeService {
    async fn get_card_grade(&self, card_number: &i32) -> GradeEnum;
}