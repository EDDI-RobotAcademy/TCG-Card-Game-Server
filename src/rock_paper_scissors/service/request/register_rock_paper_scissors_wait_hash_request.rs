#[derive(Debug)]
pub struct RegisterRockPaperScissorsWaitHashRequest {
    account_unique_id: i32,
    opponent_unique_id: i32,
    choice: String,
}

impl RegisterRockPaperScissorsWaitHashRequest {
    pub fn new(
        account_unique_id: i32,
        opponent_unique_id: i32,
        choice: String
    ) -> Self {

        RegisterRockPaperScissorsWaitHashRequest {
            account_unique_id,
            opponent_unique_id,
            choice
        }
    }

    pub fn get_account_unique_id(&self) -> i32 {
        self.account_unique_id
    }

    pub fn get_opponent_unique_id(&self) -> i32 {
        self.opponent_unique_id
    }

    pub fn get_choice(&self) -> &str {
        &self.choice
    }
}
