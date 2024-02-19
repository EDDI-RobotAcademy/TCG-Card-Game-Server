#[derive(Debug)]
pub struct ResetSupportCardUsageCountRequest {
    account_unique_id: i32
}

impl ResetSupportCardUsageCountRequest {
    pub fn new(account_unique_id: i32) -> Self { ResetSupportCardUsageCountRequest { account_unique_id } }
    pub fn get_account_unique_id(&self) -> i32 { self.account_unique_id }
}