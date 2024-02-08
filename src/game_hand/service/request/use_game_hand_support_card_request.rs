#[derive(Debug)]
pub struct UseGameHandSupportCardRequest {
    account_unique_id: i32,
    support_card_number: i32,
}

impl UseGameHandSupportCardRequest {
    pub fn new(account_unique_id: i32, support_card_number: i32) -> Self {
        UseGameHandSupportCardRequest {
            account_unique_id,
            support_card_number,
        }
    }

    pub fn get_account_unique_id(&self) -> i32 {
        self.account_unique_id
    }

    pub fn get_support_card_number(&self) -> i32 {
        self.support_card_number
    }
}