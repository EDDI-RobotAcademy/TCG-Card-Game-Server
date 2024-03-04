#[derive(Debug)]
pub struct CheckDrawChoiceRequest {
    account_unique_id: i32,
    opponent_unique_id: i32,
}

impl CheckDrawChoiceRequest {
    pub fn new(account_unique_id: i32,
               opponent_unique_id: i32,) -> Self {

        CheckDrawChoiceRequest {
            account_unique_id,
            opponent_unique_id,

        }
    }

    pub fn get_account_unique_id(&self) -> i32 {
        self.account_unique_id
    }

    pub fn get_opponent_unique_id(&self) -> i32 {
        self.opponent_unique_id
    }
}
