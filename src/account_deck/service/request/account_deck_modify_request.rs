#[derive(Debug)]
pub struct AccountDeckModifyRequest {
    deck_id: i32,
    account_id: String,
    deck_name: String,
}

impl AccountDeckModifyRequest {
    pub fn new(deck_id: i32, account_id: String, deck_name: String) -> Self {
        AccountDeckModifyRequest {
            deck_id,
            account_id: account_id.to_string(),
            deck_name: deck_name.to_string()
        }
    }
    pub fn deck_id(&self) -> i32 { self.deck_id }
    pub fn account_id(&self) -> &str { &self.account_id }
    pub fn deck_name(&self) -> &str { &self.deck_name }
}
