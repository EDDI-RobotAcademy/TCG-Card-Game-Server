#[derive(Debug)]
pub struct NotifyToOpponentYouUseEnergyBoostCardRequest {
    opponent_unique_id: i32,
    unit_card_index: i32,
    usage_hand_card_id: i32,
    boosting_energy_count: i32,
    boosting_energy_card_id: i32,
}

impl NotifyToOpponentYouUseEnergyBoostCardRequest {
    pub fn new(
        opponent_unique_id: i32,
        unit_card_index: i32,
        usage_hand_card_id: i32,
        boosting_energy_count: i32,
        boosting_energy_card_id: i32) -> Self {

            NotifyToOpponentYouUseEnergyBoostCardRequest {
                opponent_unique_id,
                unit_card_index,
                usage_hand_card_id,
                boosting_energy_count,
                boosting_energy_card_id
            }
    }

    pub fn get_opponent_unique_id(&self) -> i32 {
        self.opponent_unique_id
    }

    pub fn get_unit_card_index(&self) -> i32 {
        self.unit_card_index
    }

    pub fn get_usage_hand_card_id(&self) -> i32 {
        self.usage_hand_card_id
    }

    pub fn get_boosting_energy_count(&self) -> i32 {
        self.boosting_energy_count
    }

    pub fn get_boosting_energy_card_id(&self) -> i32 {
        self.boosting_energy_card_id
    }
}
