#[derive(Debug)]
pub struct GenerateUseSupportCardToRemoveYourFieldEnergyDataRequest {
    used_hand_card_id: i32,
    remaining_field_energy: i32
}

impl GenerateUseSupportCardToRemoveYourFieldEnergyDataRequest {
    pub fn new(used_hand_card_id: i32,
               remaining_field_energy: i32) -> Self {
        GenerateUseSupportCardToRemoveYourFieldEnergyDataRequest {
            used_hand_card_id,
            remaining_field_energy
        }
    }

    pub fn get_used_hand_card_id(&self) -> i32 { self.used_hand_card_id }

    pub fn get_remaining_field_energy(&self) -> i32 { self.remaining_field_energy }
}