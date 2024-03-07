#[derive(Debug)]
pub struct SetMainCharacterAsDeathRequest {
    account_unique_id: i32,
}

impl SetMainCharacterAsDeathRequest {
    pub fn new(account_unique_id: i32) -> Self {
        SetMainCharacterAsDeathRequest {
            account_unique_id
        }
    }
    pub fn get_account_unique_id(&self) -> i32 { self.account_unique_id }
}