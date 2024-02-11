use async_trait::async_trait;
use crate::card_grade::service::request::get_cards_below_target_grade_request::GetCardsBelowTargetGradeRequest;
use crate::card_grade::service::response::get_cards_below_target_grade_response::GetCardsBelowTargetGradeResponse;
use crate::common::card_attributes::card_grade::card_grade_enum::GradeEnum;

#[async_trait]
pub trait CardGradeService {
    async fn get_card_grade(&self, card_number: &i32) -> GradeEnum;
    async fn get_cards_below_target_grade(
        &self, get_cards_below_target_grade_request: GetCardsBelowTargetGradeRequest) -> GetCardsBelowTargetGradeResponse;
}