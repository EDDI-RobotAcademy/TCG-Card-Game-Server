#[derive(Debug)]
pub struct FindDeckCardIdByIndexRequest {
    account_unique_id: i32,
    target_card_index: i32,
}

impl FindDeckCardIdByIndexRequest {
    pub fn new(account_unique_id: i32,
               target_card_index: i32,) -> Self {
        FindDeckCardIdByIndexRequest {
            account_unique_id,
            target_card_index,
        }
    }
    pub fn get_account_unique_id(&self) -> i32 { self.account_unique_id }
    pub fn get_target_card_index(&self) -> i32 { self.target_card_index }
}