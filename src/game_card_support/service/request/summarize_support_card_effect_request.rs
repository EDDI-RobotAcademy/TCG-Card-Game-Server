#[derive(Clone)]
pub struct SummarizeSupportCardEffectRequest {
    support_card_number: i32,
}

impl SummarizeSupportCardEffectRequest {
    pub fn new(support_card_number: i32) -> Self {
        SummarizeSupportCardEffectRequest {
            support_card_number
        }
    }

    pub fn get_support_card_number(&self) -> i32 {
        self.support_card_number
    }
}
