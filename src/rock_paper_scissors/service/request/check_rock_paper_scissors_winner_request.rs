#[derive(Debug)]
pub struct CheckRockPaperScissorsWinnerRequest {
    account_unique_id: i32,
    opponent_unique_id: i32,
}

impl CheckRockPaperScissorsWinnerRequest {
    pub fn new(
        account_unique_id: i32,
        opponent_unique_id: i32) -> Self {

        CheckRockPaperScissorsWinnerRequest {
            account_unique_id,
            opponent_unique_id,
        }
    }

    pub fn get_account_unique_id(&self) -> i32 {
        self.account_unique_id
    }

    pub fn get_opponent_unique_id(&self) -> i32 {
        self.opponent_unique_id
    }
}
