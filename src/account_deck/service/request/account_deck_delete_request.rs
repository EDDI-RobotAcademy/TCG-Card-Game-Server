#[derive(Debug)]
pub struct AccountDeckDeleteRequest {
    deck_unique_id: i32,
    account_session_id: String
}

impl AccountDeckDeleteRequest {
    pub fn new(deck_unique_id: i32, account_session_id: String) -> Self {
        AccountDeckDeleteRequest {
            deck_unique_id,
            account_session_id
        }
    }
    pub fn deck_unique_id(&self) -> i32 { self.deck_unique_id }
    pub fn account_session_id(&self) -> &str { &self.account_session_id }
}