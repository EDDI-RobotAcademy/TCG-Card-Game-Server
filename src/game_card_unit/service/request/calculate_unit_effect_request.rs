#[derive(Clone)]
pub struct CalculateUnitEffectRequest {
    unit_card_number: i32,
}

impl CalculateUnitEffectRequest {
    pub fn new(unit_card_number: i32) -> Self {
        CalculateUnitEffectRequest {
            unit_card_number
        }
    }

    pub fn get_unit_card_number(&self) -> i32 {
        self.unit_card_number
    }
}
