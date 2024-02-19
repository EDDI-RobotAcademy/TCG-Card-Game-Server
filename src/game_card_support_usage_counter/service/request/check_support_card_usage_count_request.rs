#[derive(Debug)]
pub struct CheckSupportCardUsageCountRequest {
    account_unique_id: i32
}

impl CheckSupportCardUsageCountRequest {
    pub fn new(account_unique_id: i32) -> Self { CheckSupportCardUsageCountRequest { account_unique_id } }
    pub fn get_account_unique_id(&self) -> i32 { self.account_unique_id }
}