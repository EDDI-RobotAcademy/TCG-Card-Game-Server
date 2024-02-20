
#[derive(Debug)]
pub struct DataToDisplayInShopRequest {
    account_session_id: String,
}

impl DataToDisplayInShopRequest {
    pub fn new(account_session_id: String) -> Self {
        DataToDisplayInShopRequest { account_session_id: account_session_id.to_string() }
    }
    pub fn account_session_id(&self) -> &str { &self.account_session_id }
}