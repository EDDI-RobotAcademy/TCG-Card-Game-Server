#[derive(Debug)]
pub struct AccountCardListRequest {
    account_id: String
}

impl AccountCardListRequest {
    pub fn new(account_id: String) -> Self {
        AccountCardListRequest {
            account_id: account_id.to_string()
        }
    }
    pub fn account_id(&self) -> &str { &self.account_id }
}