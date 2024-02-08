use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DrawSupportResponseForm {
    drawn_card_list: Vec<i32>
}

impl DrawSupportResponseForm {
    pub fn new(drawn_card_list: Vec<i32>) -> Self { DrawSupportResponseForm { drawn_card_list } }
}