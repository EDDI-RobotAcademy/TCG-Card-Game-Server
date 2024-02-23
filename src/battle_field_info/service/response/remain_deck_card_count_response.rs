use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RemainDeckCardCountResponse {
    remain_deck_card_count: i32,
}

impl RemainDeckCardCountResponse {
    pub fn new(remain_deck_card_count: i32) -> Self {
        RemainDeckCardCountResponse {
            remain_deck_card_count,
        }
    }

    pub fn get_remain_deck_card_count(&self) -> i32 {
        self.remain_deck_card_count
    }
}