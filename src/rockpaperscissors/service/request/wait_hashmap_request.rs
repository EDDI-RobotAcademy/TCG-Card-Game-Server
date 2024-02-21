#[derive(Debug)]
pub struct WaitHashmapRequest {
    account_unique_id: i32,
    opponent_id:i32,
    choice: String,

}

impl WaitHashmapRequest {
    pub fn new(account_unique_id: i32,
               opponent_id:i32,
               choice: String,) -> Self {
        WaitHashmapRequest {
            account_unique_id,
            opponent_id,
            choice
        }
    }

    pub fn get_account_unique_id(&self) -> i32 {
        self.account_unique_id
    }
    pub fn get_opponent_id(&self) -> i32 {
        self.opponent_id
    }

    pub fn get_choice(&self) -> &str {
        &self.choice
    }
}
