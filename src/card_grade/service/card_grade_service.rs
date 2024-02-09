use std::collections::HashMap;
use async_trait::async_trait;
use crate::common::card_attributes::card_grade::card_grade_enum::GradeEnum;

#[async_trait]
pub trait CardGradeService {
    async fn get_card_grade(&self, card_number: &i32) -> GradeEnum;
    async fn get_cards_below_target_grade(&self, card_list: Vec<i32>, target_grade: GradeEnum) -> Vec<i32>;
}