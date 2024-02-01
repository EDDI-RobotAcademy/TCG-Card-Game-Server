#[derive(Debug)]
pub struct GainGoldRequest {
    account_id: String,
    gold: String,
}

impl GainGoldRequest {
    pub fn new(account_id: i32, gold: i32) -> Self {
        GainGoldRequest {
            account_id: account_id.to_string(),
            gold: gold.to_string()
        }
    }

    pub fn account_id(&self) -> &str { &self.account_id }

    pub fn gold(&self) -> &str { &self.gold }
}