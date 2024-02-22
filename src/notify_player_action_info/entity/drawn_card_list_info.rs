use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DrawnCardListInfo {
    drawn_card_list: Vec<i32>
}

impl DrawnCardListInfo {
    pub fn new(drawn_card_list: Vec<i32>) -> Self {
        DrawnCardListInfo {
            drawn_card_list
        }
    }
}