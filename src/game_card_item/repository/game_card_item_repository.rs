use crate::game_card_item::service::request::use_item_card_request::UseItemCardRequest;
use crate::game_card_item::service::response::use_item_card_response::UseItemCardResponse;

pub trait GameCardItemRepository {
    unsafe fn call_item_card_repository_table(&self, use_item_card_request: UseItemCardRequest) -> UseItemCardResponse;
}