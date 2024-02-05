use crate::game_card_item::handler::game_card_item_handler::GameCardItemHandler;
use crate::game_card_item::service::request::use_item_card_request::UseItemCardRequest;
use crate::game_card_item::service::response::use_item_card_response::UseItemCardResponse;

pub struct ItemCard_8_Function;

impl GameCardItemHandler for ItemCard_8_Function {
    unsafe fn use_item_card(&self, use_item_card_request: UseItemCardRequest) -> UseItemCardResponse {
        println!("ItemCard_8_Function: use_item_card()");

        UseItemCardResponse
    }
}