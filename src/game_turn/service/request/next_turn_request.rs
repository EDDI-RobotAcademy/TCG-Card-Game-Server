#[derive(Debug)]
pub struct NextTurnRequest {
    account_unique_id: i32,
}

impl NextTurnRequest {
    pub fn new(account_unique_id: i32) -> Self {
        NextTurnRequest {
            account_unique_id
        }
    }

    pub fn get_account_unique_id(&self) -> i32 {
        self.account_unique_id
    }
}
