#[derive(Debug)]
pub struct PayGoldRequest {
    account_id: i32,
    gold: i32,
}

impl PayGoldRequest {
    pub fn new(account_id: i32, gold: i32) -> Self {
        PayGoldRequest {
            account_id,
            gold,
        }
    }

    pub fn account_id(&self) -> i32 { self.account_id }

    pub fn gold(&self) -> i32 { self.gold }
}