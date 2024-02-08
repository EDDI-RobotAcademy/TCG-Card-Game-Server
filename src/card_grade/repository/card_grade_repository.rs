use std::collections::HashMap;
use async_trait::async_trait;
use crate::common::card_attributes::card_grade::card_grade_enum::GradeEnum;

#[async_trait]
pub trait CardGradeRepository {
    async fn get_card_grade(&self, card_number: &i32) -> GradeEnum;
    async fn get_grade_by_specific_race_card_list(&self, race_specific_card_id_list: Vec<i32>) -> Vec<(i32,GradeEnum)>;
    async fn get_grade_by_card_list(&self, card_list: Vec<i32>) -> HashMap<i32, GradeEnum>;


}