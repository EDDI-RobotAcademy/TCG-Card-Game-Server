#[derive(Clone)]
pub struct SummaryItemCardEffectRequest {
    item_card_id: i32,
}

impl SummaryItemCardEffectRequest {
    pub fn new(item_card_id: i32) -> Self {
        SummaryItemCardEffectRequest {
            item_card_id
        }
    }

    pub fn get_item_card_id(&self) -> i32 {
        self.item_card_id
    }
}
