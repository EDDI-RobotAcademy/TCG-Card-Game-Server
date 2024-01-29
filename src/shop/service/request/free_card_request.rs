#[derive(Debug)]
pub struct FreeCardRequest {
    account_id: String,
}

impl FreeCardRequest {
    pub fn new(account_id: String) -> Self {
        FreeCardRequest { account_id: account_id.to_string() }
    }
    pub fn account_id(&self) -> &str { &self.account_id }
}