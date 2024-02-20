use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataToDisplayInShopResponse {
    account_point: i32
}

impl DataToDisplayInShopResponse {
    pub fn new(account_point: i32) -> Self {
        DataToDisplayInShopResponse { account_point }
    }
    pub fn get_account_point(&self) -> i32 { self.account_point }
}