#[derive(Debug)]
pub struct PayGoldRequest {
    account_id: String,
    gold: String,
}

impl PayGoldRequest {
    pub fn new(account_id: i32, gold: i32) -> Self {
        PayGoldRequest {
            account_id: account_id.to_string(),
            gold: gold.to_string()
        }
    }

    pub fn account_id(&self) -> &str { &self.account_id }

    pub fn gold(&self) -> &str { &self.gold }
}