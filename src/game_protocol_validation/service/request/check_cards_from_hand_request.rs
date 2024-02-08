#[derive(Debug)]
pub struct CheckCardsFromHandRequest {
    account_session_id: String,
    hand_card_list: Vec<i32>,
}

impl CheckCardsFromHandRequest {
    pub fn new(account_session_id: &str, hand_card_list: Vec<i32>) -> Self {
        CheckCardsFromHandRequest {
            account_session_id: account_session_id.to_string(),
            hand_card_list
        }
    }
    pub fn get_account_session_id(&self) -> &str { &self.account_session_id }
    pub fn get_hand_card_list(&self) -> &Vec<i32> { &self.hand_card_list }
}