#[derive(Debug)]
pub struct NoticeSearchCardRequest {
    opponent_unique_id: i32,
    found_card_list: Vec<i32>,
}

impl NoticeSearchCardRequest {
    pub fn new(opponent_unique_id: i32,
               found_card_list: Vec<i32>,) -> Self {
        NoticeSearchCardRequest {
            opponent_unique_id,
            found_card_list
        }
    }

    pub fn get_opponent_unique_id(&self) -> i32 { self.opponent_unique_id }

    pub fn get_found_card_list(&self) -> &Vec<i32> { &self.found_card_list }
}