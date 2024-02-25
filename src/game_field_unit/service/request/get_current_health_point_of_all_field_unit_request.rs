#[derive(Debug)]
pub struct GetCurrentHealthPointOfAllFieldUnitRequest {
    account_unique_id: i32,
}

impl GetCurrentHealthPointOfAllFieldUnitRequest {
    pub fn new(account_unique_id: i32) -> Self {
        GetCurrentHealthPointOfAllFieldUnitRequest {
            account_unique_id,
        }
    }
    pub fn get_account_unique_id(&self) -> i32 {
        self.account_unique_id
    }
}