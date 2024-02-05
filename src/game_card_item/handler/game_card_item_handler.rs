use crate::game_card_item::service::request::use_item_card_request::UseItemCardRequest;
use crate::game_card_item::service::response::use_item_card_response::UseItemCardResponse;

pub trait GameCardItemHandler {
    unsafe fn use_item_card(&self, use_item_card_request: UseItemCardRequest) -> UseItemCardResponse;
}