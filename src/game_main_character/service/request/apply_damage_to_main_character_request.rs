#[derive(Debug)]
pub struct ApplyDamageToMainCharacterRequest {
    account_unique_id: i32,
    damage: i32,
}

impl ApplyDamageToMainCharacterRequest {
    pub fn new(account_unique_id: i32, damage: i32) -> Self {
        ApplyDamageToMainCharacterRequest {
            account_unique_id,
            damage,
        }
    }
    pub fn get_account_unique_id(&self) -> i32 { self.account_unique_id }
    pub fn get_damage(&self) -> i32 { self.damage }
}