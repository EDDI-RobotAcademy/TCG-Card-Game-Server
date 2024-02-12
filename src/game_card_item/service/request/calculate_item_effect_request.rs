#[derive(Clone)]
pub struct CalculateItemEffectRequest {
    item_card_number: i32,
}

impl CalculateItemEffectRequest {
    pub fn new(item_card_number: i32) -> Self {
        CalculateItemEffectRequest {
            item_card_number
        }
    }

    pub fn get_item_card_number(&self) -> i32 {
        self.item_card_number
    }
}
