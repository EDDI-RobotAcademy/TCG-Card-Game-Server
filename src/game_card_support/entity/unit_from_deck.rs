use crate::common::card_attributes::card_grade::card_grade_enum::GradeEnum;

#[derive(PartialEq, Debug)]
pub struct UnitFromDeck {
    grade_limit: GradeEnum,
    unit_count: i32,
}

impl UnitFromDeck {
    pub fn new(grade_limit: GradeEnum, unit_count: i32) -> Self {
        UnitFromDeck {
            grade_limit,
            unit_count }
    }
    pub fn set_grade_limit(&mut self, grade_limit: GradeEnum) { self.grade_limit = grade_limit }
    pub fn set_unit_count(&mut self, unit_count: i32) { self.unit_count = unit_count }
    pub fn get_grade_limit(&self) -> GradeEnum { self.grade_limit }
    pub fn get_unit_count(&self) -> i32 { self.unit_count }
}