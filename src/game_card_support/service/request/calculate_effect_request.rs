use crate::game_hand::service::request::use_game_hand_unit_card_request::UseGameHandUnitCardRequest;

#[derive(Clone)]
pub struct CalculateEffectRequest {
    support_card_number: i32,
}

impl CalculateEffectRequest {
    pub fn new(support_card_number: i32) -> Self {
        CalculateEffectRequest {
            support_card_number
        }
    }

    pub fn get_support_card_number(&self) -> i32 {
        self.support_card_number
    }
}
