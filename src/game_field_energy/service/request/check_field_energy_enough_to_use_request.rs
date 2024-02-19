#[derive(Debug)]
pub struct CheckFieldEnergyEnoughToUseRequest {
    account_unique_id: i32,
    will_be_used_amount: i32,
}

impl CheckFieldEnergyEnoughToUseRequest {
    pub fn new(account_unique_id: i32, will_be_used_amount: i32) -> Self {
        CheckFieldEnergyEnoughToUseRequest {
            account_unique_id,
            will_be_used_amount
        }
    }
    pub fn get_account_unique_id(&self) -> i32 { self.account_unique_id }

    pub fn get_will_be_used_amount(&self) -> i32 { self.will_be_used_amount }
}