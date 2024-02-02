#[derive(Debug)]
pub struct GetCardDefaultRequest {
    account_id: String,
}

impl GetCardDefaultRequest {
    pub fn new(account_id: String) -> Self {
        GetCardDefaultRequest { account_id: account_id.to_string() }
    }
    pub fn account_id(&self) -> &str { &self.account_id }
}