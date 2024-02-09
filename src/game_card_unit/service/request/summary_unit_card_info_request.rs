#[derive(Clone)]
pub struct SummaryUnitCardInfoRequest {
    unit_card_id: i32,
}

impl SummaryUnitCardInfoRequest {
    pub fn new(unit_card_id: i32) -> Self {
        SummaryUnitCardInfoRequest {
            unit_card_id
        }
    }

    pub fn get_unit_card_id(&self) -> i32 {
        self.unit_card_id
    }
}
