#[derive(Debug)]
pub struct AddCardListToHandRequest {
    account_unique_id: i32,
    card_list: Vec<i32>,
}

impl AddCardListToHandRequest {
    pub fn new(account_unique_id: i32, card_list: Vec<i32>) -> Self {
        AddCardListToHandRequest {
            account_unique_id,
            card_list,
        }
    }

    pub fn get_account_unique_id(&self) -> i32 {
        self.account_unique_id
    }

    pub fn get_card_list(&self) -> &Vec<i32> {
        &self.card_list
    }
}