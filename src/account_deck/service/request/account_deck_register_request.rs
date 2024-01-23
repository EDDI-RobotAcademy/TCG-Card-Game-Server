#[derive(Debug)]
pub struct AccountDeckRegisterRequest {
    account_id: String,
    deck_name: String,
}

impl AccountDeckRegisterRequest {
    pub fn new(account_id: String, deck_name: String) -> Self {
        AccountDeckRegisterRequest {
            account_id: account_id.to_string(),
            deck_name: deck_name.to_string()
        }
    }

    pub fn account_id(&self) -> &str { &self.account_id }

    pub fn deck_name(&self) -> &str { &self.deck_name }
}

