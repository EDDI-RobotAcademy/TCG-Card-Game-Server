use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameDeckStartCardListResponse {
    is_success: bool,
    hand_card_list: Vec<i32>,
}

impl GameDeckStartCardListResponse {
    pub fn new(is_success: bool, hand_card_list: Vec<i32>) -> Self {
        GameDeckStartCardListResponse {
            is_success,
            hand_card_list,
        }
    }

    pub fn default() -> GameDeckStartCardListResponse {
        GameDeckStartCardListResponse::new(
            false,
            vec![]
        )
    }
}
