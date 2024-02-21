#[derive(Debug)]
pub struct JudgeDeathOfEveryUnitRequest {
    account_unique_id: i32,
}

impl JudgeDeathOfEveryUnitRequest {
    pub fn new(account_unique_id: i32) -> Self {
        JudgeDeathOfEveryUnitRequest {
            account_unique_id,
        }
    }

    pub fn get_account_unique_id(&self) -> i32 {
        self.account_unique_id
    }
}