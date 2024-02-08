use crate::game_card_item::entity::game_card_item_effect::GameCardItemEffect;
use crate::game_card_item::service::request::use_item_card_request::UseItemCardRequest;
use crate::game_card_item::service::response::use_item_card_response::UseItemCardResponse;

pub trait GameCardItemRepository {
    unsafe fn call_item_card_repository_handler(&self, item_card_id: i32) -> GameCardItemEffect;
}