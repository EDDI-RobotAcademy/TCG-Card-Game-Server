#[derive(Debug)]
pub struct ApplyStatusEffectDamageIterativelyRequest {
    account_unique_id: i32,
}

impl ApplyStatusEffectDamageIterativelyRequest {
    pub fn new(account_unique_id: i32) -> Self {
        ApplyStatusEffectDamageIterativelyRequest {
            account_unique_id,
        }
    }

    pub fn get_account_unique_id(&self) -> i32 {
        self.account_unique_id
    }
}
