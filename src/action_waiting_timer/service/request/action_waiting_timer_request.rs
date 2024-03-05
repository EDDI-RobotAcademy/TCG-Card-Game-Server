#[derive(Debug)]
pub struct ActionWaitingTimerRequest {
    account_unique_id: i32,
}

impl ActionWaitingTimerRequest {
    pub fn new(account_unique_id: i32) -> Self {
        ActionWaitingTimerRequest {
            account_unique_id,
        }
    }

    pub fn get_account_unique_id(&self) -> i32 {
        self.account_unique_id
    }
}