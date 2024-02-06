#[derive(Debug)]
pub struct PutCardsOnDeckRequest {
    session_id: String,
    will_be_changed_card_list: Vec<String>,
}

impl PutCardsOnDeckRequest {
    pub fn new(session_id: String, will_be_changed_card_list: Vec<String>) -> Self {
        PutCardsOnDeckRequest {
            session_id,
            will_be_changed_card_list
        }
    }
    pub fn get_session_id(&self) -> &str { &self.session_id }
    pub fn get_will_be_changed_card_list(&self) -> &Vec<String> { &self.will_be_changed_card_list }
}