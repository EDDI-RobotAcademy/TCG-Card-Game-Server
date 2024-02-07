#[derive(Debug)]
pub struct SupportCardProtocolValidationRequest {
    account_unique_id: i32,
    support_card_number: String,
}

impl SupportCardProtocolValidationRequest {
    pub fn new(account_unique_id: i32, support_card_number: String) -> Self {
        SupportCardProtocolValidationRequest {
            account_unique_id,
            support_card_number: support_card_number.to_string(),
        }
    }

    pub fn get_support_card_number(&self) -> &str {
        &self.support_card_number
    }

    pub fn get_account_unique_id(&self) -> i32 {
        self.account_unique_id
    }
}