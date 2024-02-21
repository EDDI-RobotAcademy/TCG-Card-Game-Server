#[derive(Debug)]
pub struct ResetTurnActionOfAllFieldUnitRequest {
    account_unique_id: i32,
}

impl ResetTurnActionOfAllFieldUnitRequest {
    pub fn new(account_unique_id: i32) -> Self {
        ResetTurnActionOfAllFieldUnitRequest {
            account_unique_id,
        }
    }

    pub fn get_account_unique_id(&self) -> i32 {
        self.account_unique_id
    }
}