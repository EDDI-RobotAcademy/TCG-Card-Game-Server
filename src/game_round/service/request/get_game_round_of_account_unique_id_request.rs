#[derive(Debug)]
pub struct GetGameRoundOfAccountUniqueIdRequest {
    account_unique_id: i32,
}

impl GetGameRoundOfAccountUniqueIdRequest {
    pub fn new(account_unique_id: i32) -> Self {
        GetGameRoundOfAccountUniqueIdRequest {
            account_unique_id,
        }
    }

    pub fn get_account_unique_id(&self) -> i32 {
        self.account_unique_id
    }
}