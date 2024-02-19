#[derive(Debug)]
pub struct UpdateSupportCardUsageCountRequest {
    account_unique_id: i32
}

impl UpdateSupportCardUsageCountRequest {
    pub fn new(account_unique_id: i32) -> Self { UpdateSupportCardUsageCountRequest { account_unique_id } }
    pub fn get_account_unique_id(&self) -> i32 { self.account_unique_id }
}