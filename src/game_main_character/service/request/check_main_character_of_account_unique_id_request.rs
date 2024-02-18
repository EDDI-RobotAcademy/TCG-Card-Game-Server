#[derive(Debug)]
pub struct CheckMainCharacterOfAccountUniqueIdRequest {
    account_unique_id: i32,
}

impl CheckMainCharacterOfAccountUniqueIdRequest {
    pub fn new(account_unique_id: i32) -> Self {
        CheckMainCharacterOfAccountUniqueIdRequest {
            account_unique_id,
        }
    }

    pub fn get_account_unique_id(&self) -> i32 {
        self.account_unique_id
    }
}