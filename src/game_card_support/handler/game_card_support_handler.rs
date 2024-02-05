use crate::game_card_support::service::request::use_support_card_request::UseSupportCardRequest;
use crate::game_card_support::service::response::use_support_card_response::UseSupportCardResponse;

pub trait SupportCardSupportHandler {
    unsafe fn use_support_card(&self, use_support_card_request: UseSupportCardRequest) -> UseSupportCardResponse;
}