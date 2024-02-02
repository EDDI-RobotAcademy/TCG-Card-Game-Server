#[derive(Debug)]
pub struct GetCardDafaultRequest {
    account_id: String,
}

impl GetCardDafaultRequest {
    pub fn new(account_id: String) -> Self {
        GetCardDafaultRequest { account_id: account_id.to_string() }
    }
    pub fn account_id(&self) -> &str { &self.account_id }
}