#[derive(Debug)]
pub struct GenerateOpponentDeckCardLostDataRequest {
    lost_deck_card_list: Vec<i32>,
}

impl GenerateOpponentDeckCardLostDataRequest {
    pub fn new(lost_deck_card_list: Vec<i32>) -> Self {
        GenerateOpponentDeckCardLostDataRequest {
            lost_deck_card_list
        }
    }

    pub fn get_lost_deck_card_list(&self) -> &Vec<i32> { &self.lost_deck_card_list }
}