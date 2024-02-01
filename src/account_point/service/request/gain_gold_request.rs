#[derive(Debug)]
pub struct GainGoldRequest {
    account_id: i32,
    gold: i32,
}

impl GainGoldRequest {
    pub fn new(account_id: i32, gold: i32) -> Self {
        GainGoldRequest {
            account_id,
            gold,
        }
    }

    pub fn account_id(&self) -> i32 { self.account_id }

    pub fn gold(&self) -> i32 { self.gold }
}