use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecuteFreeGachaResponseForm {
    card_id_list: Vec<i32>,
    is_success: bool
}

impl ExecuteFreeGachaResponseForm {
    pub fn new(card_id_list: Vec<i32>, is_success: bool) -> Self { ExecuteFreeGachaResponseForm { card_id_list, is_success } }
    pub fn get_card_id_list(&self) -> &Vec<i32> { &self.card_id_list }
    pub fn is_success(&self) -> bool { self.is_success }

}