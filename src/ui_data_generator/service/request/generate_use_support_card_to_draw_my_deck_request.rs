#[derive(Debug)]
pub struct GenerateUseSupportCardToDrawMyDeckRequest {
    used_hand_card_id: i32,
    drawn_card_list: Vec<i32>,
}

impl GenerateUseSupportCardToDrawMyDeckRequest {
    pub fn new(used_hand_card_id: i32,
               drawn_card_list: Vec<i32>,) -> Self {
        GenerateUseSupportCardToDrawMyDeckRequest {
            used_hand_card_id,
            drawn_card_list,
        }
    }

    pub fn get_used_hand_card_id(&self) -> i32 { self.used_hand_card_id }

    pub fn get_drawn_card_list(&self) -> &Vec<i32> { &self.drawn_card_list }
}