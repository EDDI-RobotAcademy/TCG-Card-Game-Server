#[derive(Debug)]
pub struct NoticeDrawCardRequest {
    opponent_unique_id: i32,
    drawn_card_list: Vec<i32>,
}

impl NoticeDrawCardRequest {
    pub fn new(opponent_unique_id: i32,
               drawn_card_list: Vec<i32>) -> Self {
        NoticeDrawCardRequest {
            opponent_unique_id,
            drawn_card_list
        }
    }

    pub fn get_opponent_unique_id(&self) -> i32 { self.opponent_unique_id }

    pub fn get_drawn_card_list(&self) -> &Vec<i32> { &self.drawn_card_list }
}