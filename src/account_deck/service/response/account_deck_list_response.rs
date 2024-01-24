use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccountDeckListResponse {
    account_deck_list: Vec<HashMap<i32, String>>
}

impl AccountDeckListResponse {
    pub fn new(account_deck_list: Vec<HashMap<i32, String>>) -> Self {
        AccountDeckListResponse { account_deck_list }
    }
    pub fn get_list(&self) -> &Vec<HashMap<i32, String>> { &self.account_deck_list }
}

#[cfg(test)]
mod test {
    use super::*;
    use tokio::test;
    #[tokio::test]
    async fn test_account_deck_list_response_generation() {

        let mut sample_deck_map: HashMap<i32, String> = HashMap::new();
        sample_deck_map.insert(1, "test_deck1".to_string());
        sample_deck_map.insert(2, "test_deck2".to_string());
        sample_deck_map.insert(3, "test_deck3".to_string());

        let mut sample_deck_list: Vec<HashMap<i32, String>> = Vec::new();
        sample_deck_list.push(sample_deck_map);

        let sample_response = AccountDeckListResponse::new(sample_deck_list.clone());
        for maps in sample_deck_list {
            for (key, value) in maps {
                println!("{}, {}", key, value);
            }
        }
        let what = sample_response.get_list();
        println!("{:?}", what)
    }
    #[tokio::test]
    #[cfg(not(feature = "deck_registration_test"))]
    async fn dummy_test() {
        assert!(true);
    }
}

