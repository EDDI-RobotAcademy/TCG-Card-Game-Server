use crate::game_card_support::service::request::use_support_card_request::UseSupportCardRequest;
use crate::game_card_support::service::response::use_support_card_response::UseSupportCardResponse;

pub trait GameCardSupportRepository {
    unsafe fn call_support_card_repository_table(&self, use_support_card_request: UseSupportCardRequest) -> UseSupportCardResponse;
}