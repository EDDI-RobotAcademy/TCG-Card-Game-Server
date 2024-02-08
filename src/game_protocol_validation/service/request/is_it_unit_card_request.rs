#[derive(Debug)]
pub struct IsItUnitCardRequest {
    unit_card_id: i32,
}

impl IsItUnitCardRequest {
    pub fn new(unit_card_id: i32) -> Self {
        IsItUnitCardRequest {
            unit_card_id
        }
    }

    pub fn get_unit_card_id(&self) -> i32 {
        self.unit_card_id
    }
}
