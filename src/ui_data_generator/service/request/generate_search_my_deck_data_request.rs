#[derive(Debug)]
pub struct GenerateSearchMyDeckDataRequest {
    used_hand_card_id: i32,
    found_card_list: Vec<i32>,
}

impl GenerateSearchMyDeckDataRequest {
    pub fn new(used_hand_card_id: i32,
               found_card_list: Vec<i32>,) -> Self {
        GenerateSearchMyDeckDataRequest {
            used_hand_card_id,
            found_card_list
        }
    }

    pub fn get_used_hand_card_id(&self) -> i32 { self.used_hand_card_id }

    pub fn get_found_card_list(&self) -> &Vec<i32> { &self.found_card_list }
}