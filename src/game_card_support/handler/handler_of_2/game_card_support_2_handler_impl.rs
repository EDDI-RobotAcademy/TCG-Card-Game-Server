use crate::game_card_support::handler::game_card_support_handler::GameCardSupportHandler;
use crate::game_card_support::service::request::use_support_card_request::UseSupportCardRequest;
use crate::game_card_support::service::response::use_support_card_response::UseSupportCardResponse;

pub struct SupportCard_2_Function;

impl GameCardSupportHandler for SupportCard_2_Function {
    unsafe fn use_support_card(&self, use_support_card_request: UseSupportCardRequest) -> UseSupportCardResponse {
        println!("SupportCard_2_Function: use_support_card()");

        UseSupportCardResponse
    }
}