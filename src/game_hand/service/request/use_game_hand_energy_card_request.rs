#[derive(Debug)]
pub struct UseGameHandEnergyCardRequest {
    account_unique_id: i32,
    energy_card_id: i32,
}

impl UseGameHandEnergyCardRequest {
    pub fn new(account_unique_id: i32, energy_card_id: i32) -> Self {
        UseGameHandEnergyCardRequest {
            account_unique_id,
            energy_card_id,
        }
    }

    pub fn get_account_unique_id(&self) -> i32 {
        self.account_unique_id
    }

    pub fn get_energy_card_id(&self) -> i32 {
        self.energy_card_id
    }
}
