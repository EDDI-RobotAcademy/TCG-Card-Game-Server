#[derive(Debug)]
pub struct NoticeLostDeckCardOfOpponentRequest {
    opponent_unique_id: i32,
    lost_deck_card_list: Vec<i32>,
}

impl NoticeLostDeckCardOfOpponentRequest {
    pub fn new(opponent_unique_id: i32,
               lost_deck_card_list: Vec<i32>) -> Self {
        NoticeLostDeckCardOfOpponentRequest {
            opponent_unique_id,
            lost_deck_card_list
        }
    }

    pub fn get_opponent_unique_id(&self) -> i32 { self.opponent_unique_id }

    pub fn get_lost_deck_card_list(&self) -> &Vec<i32> { &self.lost_deck_card_list }
}