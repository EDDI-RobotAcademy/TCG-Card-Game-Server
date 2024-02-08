#[derive(Debug)]
pub struct FindOpponentByAccountIdRequest {
    account_unique_id: i32,
}

impl FindOpponentByAccountIdRequest {
    pub fn new(account_unique_id: i32) -> Self {
        FindOpponentByAccountIdRequest {
            account_unique_id
        }
    }

    pub fn get_account_unique_id(&self) -> i32 {
        self.account_unique_id
    }
}