#[derive(Debug)]
pub struct CheckWinnerRequest {
    account_unique_id:i32,
    opponent_id:i32,

}

impl CheckWinnerRequest {
    pub fn new(account_unique_id:i32,opponent_id:i32) -> Self {
        CheckWinnerRequest {
            account_unique_id,
            opponent_id,

        }
    }

    pub fn get_account_unique_id(&self) -> i32 {
        self.account_unique_id
    }
    pub fn get_opponent_id(&self) -> i32 {
        self.opponent_id
    }


}
