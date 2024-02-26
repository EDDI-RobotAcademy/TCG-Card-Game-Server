#[derive(Debug)]
pub struct GetCurrentMainCharacterHealthPointRequest {
    account_unique_id: i32,
}

impl GetCurrentMainCharacterHealthPointRequest {
    pub fn new(account_unique_id: i32) -> Self {
        GetCurrentMainCharacterHealthPointRequest {
            account_unique_id,
        }
    }

    pub fn get_account_unique_id(&self) -> i32 {
        self.account_unique_id
    }
}