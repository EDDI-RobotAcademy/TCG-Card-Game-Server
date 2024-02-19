#[derive(Debug)]
pub struct CheckWinnerRequest {
    account_id:i32,

}

impl CheckWinnerRequest {
    pub fn new(account_id:i32) -> Self {
        CheckWinnerRequest {
            account_id,

        }
    }

    pub fn get_account_id(&self) -> i32 {
        self.account_id
    }


}
