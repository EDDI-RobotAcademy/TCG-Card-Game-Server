#[derive(Debug)]
pub struct GetCardDefaultRequest {
    account_id: String,
    race_name: String,
}

impl GetCardDefaultRequest {
    pub fn new(account_id: String, race_name: String) -> Self {
        GetCardDefaultRequest { account_id: account_id.to_string(), race_name: race_name.to_string() }
    }
    pub fn account_id(&self) -> &str { &self.account_id }
    pub fn race_name(&self) -> &str { &self.race_name }
}