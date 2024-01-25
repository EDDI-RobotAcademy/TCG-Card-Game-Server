#[derive(Debug)]
pub struct AccountDeckListRequest {
    account_id: String
}

impl AccountDeckListRequest {
    pub fn new(account_id: String) -> Self {
        AccountDeckListRequest {
            account_id: account_id.to_string()
        }
    }

    pub fn account_id(&self) -> &str { &self.account_id }
}