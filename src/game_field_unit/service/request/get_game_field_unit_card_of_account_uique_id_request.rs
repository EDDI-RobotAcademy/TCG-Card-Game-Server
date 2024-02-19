#[derive(Debug)]
pub struct GetGameFieldUnitCardOfAccountUniqueIdRequest {
    account_unique_id: i32,
}

impl GetGameFieldUnitCardOfAccountUniqueIdRequest {
    pub fn new(account_unique_id: i32) -> Self {
        GetGameFieldUnitCardOfAccountUniqueIdRequest {
            account_unique_id,
        }
    }

    pub fn get_account_unique_id(&self) -> i32 {
        self.account_unique_id
    }
}