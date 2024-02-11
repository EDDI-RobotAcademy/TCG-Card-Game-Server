use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct GetCardsBelowTargetGradeResponse {
    filtered_card_list: Vec<i32>,
}

impl GetCardsBelowTargetGradeResponse {
    pub fn new(filtered_card_list: Vec<i32>) -> Self {
        GetCardsBelowTargetGradeResponse { filtered_card_list }
    }
    pub fn get_filtered_card_list(&self) -> &Vec<i32> { &self.filtered_card_list }
}