use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChangeFirstHandResponseForm {
    redrawn_hand_card_list: Vec<i32>,
    updated_deck_card_list: Vec<i32>,
}

impl ChangeFirstHandResponseForm {
    pub fn new(redrawn_hand_card_list: Vec<i32>, updated_deck_card_list: Vec<i32>) -> Self {
        ChangeFirstHandResponseForm {
            redrawn_hand_card_list,
            updated_deck_card_list
        }
    }
    pub fn get_redrawn_hand_card_list(&self) -> &Vec<i32> { &self.redrawn_hand_card_list }
    pub fn get_updated_deck_card_list(&self) -> &Vec<i32> { &self.updated_deck_card_list }
}