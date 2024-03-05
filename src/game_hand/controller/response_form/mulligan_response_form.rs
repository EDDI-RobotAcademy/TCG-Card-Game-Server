use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MulliganResponseForm {
    redrawn_hand_card_list: Vec<i32>,
    updated_deck_card_list: Vec<i32>,
}

impl MulliganResponseForm {
    pub fn new(redrawn_hand_card_list: Vec<i32>, updated_deck_card_list: Vec<i32>) -> Self {
        MulliganResponseForm {
            redrawn_hand_card_list,
            updated_deck_card_list
        }
    }

    pub fn default() -> MulliganResponseForm {
        MulliganResponseForm::new(Vec::new(), Vec::new())
    }

    pub fn get_redrawn_hand_card_list(&self) -> &Vec<i32> { &self.redrawn_hand_card_list }

    pub fn get_updated_deck_card_list(&self) -> &Vec<i32> { &self.updated_deck_card_list }
}