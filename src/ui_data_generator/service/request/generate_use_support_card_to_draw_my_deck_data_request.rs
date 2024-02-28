#[derive(Debug)]
pub struct GenerateUseSupportCardToDrawMyDeckDataRequest {
    used_hand_card_id: i32,
    drawn_card_list: Vec<i32>,
}

impl GenerateUseSupportCardToDrawMyDeckDataRequest {
    pub fn new(used_hand_card_id: i32,
               drawn_card_list: Vec<i32>,) -> Self {
        GenerateUseSupportCardToDrawMyDeckDataRequest {
            used_hand_card_id,
            drawn_card_list,
        }
    }

    pub fn get_used_hand_card_id(&self) -> i32 { self.used_hand_card_id }

    pub fn get_drawn_card_list(&self) -> &Vec<i32> { &self.drawn_card_list }
}