#[derive(Debug)]
pub struct IsThisYourTurnRequest {
    account_unique_id: i32,
}

impl IsThisYourTurnRequest {
    pub fn new(account_unique_id: i32) -> Self {
        IsThisYourTurnRequest {
            account_unique_id,
        }
    }

    pub fn get_account_unique_id_for_validation(&self) -> i32 { self.account_unique_id }
}