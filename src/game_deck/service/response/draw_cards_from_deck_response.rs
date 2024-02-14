use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DrawCardsFromDeckResponse {
    draw_card_list: Vec<i32>,
}

impl DrawCardsFromDeckResponse {
    pub fn new(draw_card_list: Vec<i32>) -> Self {
        DrawCardsFromDeckResponse { draw_card_list }
    }
    pub fn get_drawn_card_list(&self) -> &Vec<i32> { &self.draw_card_list }
}