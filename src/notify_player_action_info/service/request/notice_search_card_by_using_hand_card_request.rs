#[derive(Debug)]
pub struct NoticeSearchCardByUsingHandCardRequest {
    account_unique_id: i32,
    opponent_unique_id: i32,
    used_hand_card_id: i32,
    found_card_list: Vec<i32>,
}

impl NoticeSearchCardByUsingHandCardRequest {
    pub fn new(account_unique_id: i32,
               opponent_unique_id: i32,
               used_hand_card_id: i32,
               found_card_list: Vec<i32>,) -> Self {
        NoticeSearchCardByUsingHandCardRequest {
            account_unique_id,
            opponent_unique_id,
            used_hand_card_id,
            found_card_list
        }
    }

    pub fn get_account_unique_id(&self) -> i32 { self.account_unique_id }

    pub fn get_opponent_unique_id(&self) -> i32 { self.opponent_unique_id }

    pub fn get_used_hand_card_id(&self) -> i32 { self.used_hand_card_id }

    pub fn get_found_card_list(&self) -> &Vec<i32> { &self.found_card_list }
}