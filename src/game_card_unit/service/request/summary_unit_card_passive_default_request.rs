#[derive(Clone)]
pub struct SummaryUnitCardPassiveDefaultRequest {
    unit_card_id: i32,
}

impl SummaryUnitCardPassiveDefaultRequest {
    pub fn new(unit_card_id: i32) -> Self {
        SummaryUnitCardPassiveDefaultRequest {
            unit_card_id
        }
    }

    pub fn get_unit_card_id(&self) -> i32 {
        self.unit_card_id
    }
}
