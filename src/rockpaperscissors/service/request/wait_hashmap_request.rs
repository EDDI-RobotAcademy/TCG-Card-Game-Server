#[derive(Debug)]
pub struct WaitHashmapRequest {
    account_unique_id: i32,
    choice: String,

}

impl WaitHashmapRequest {
    pub fn new(account_unique_id: i32,
               choice: String,) -> Self {
        WaitHashmapRequest {
            account_unique_id,
            choice
        }
    }

    pub fn get_account_unique_id(&self) -> i32 {
        self.account_unique_id
    }

    pub fn get_choice(&self) -> &str {
        &self.choice
    }
}
