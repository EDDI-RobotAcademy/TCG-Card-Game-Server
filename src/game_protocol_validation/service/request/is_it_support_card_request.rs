#[derive(Debug)]
pub struct IsItSupportCardRequest {
    support_card_number: i32,
}

impl IsItSupportCardRequest {
    pub fn new(support_card_number: i32) -> Self {
        IsItSupportCardRequest {
            support_card_number
        }
    }

    pub fn get_support_card_number(&self) -> i32 {
        self.support_card_number
    }
}