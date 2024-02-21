#[derive(Debug)]
pub struct CheckMainCharacterRequest {
    account_unique_id: i32,
}

impl CheckMainCharacterRequest {
    pub fn new(account_unique_id: i32) -> Self {
        CheckMainCharacterRequest {
            account_unique_id
        }
    }

    pub fn get_account_unique_id(&self) -> i32 {
        self.account_unique_id
    }
}