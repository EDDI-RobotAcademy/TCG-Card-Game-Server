use crate::common::card_attributes::card_grade::card_grade_enum::GradeEnum;

#[derive(Debug)]
pub struct GetCardsBelowTargetGradeRequest {
    card_list: Vec<i32>,
    target_grade: GradeEnum,
}

impl GetCardsBelowTargetGradeRequest {
    pub fn new(card_list: Vec<i32>, target_grade: GradeEnum) -> Self {
        GetCardsBelowTargetGradeRequest {
            card_list,
            target_grade,
        }
    }
    pub fn get_card_list(&self) -> &Vec<i32> { &self.card_list }
    pub fn get_target_grade(&self) -> &GradeEnum { &self.target_grade }
}