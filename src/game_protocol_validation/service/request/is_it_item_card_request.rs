#[derive(Debug)]
pub struct IsItItemCardRequest {
    item_card_id: i32,
}

impl IsItItemCardRequest {
    pub fn new(item_card_id: i32) -> Self {
        IsItItemCardRequest {
            item_card_id
        }
    }

    pub fn get_item_card_id(&self) -> i32 {
        self.item_card_id
    }
}
