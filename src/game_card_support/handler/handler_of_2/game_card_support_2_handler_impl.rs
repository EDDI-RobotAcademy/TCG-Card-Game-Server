use crate::common::card_attributes::card_race::card_race_enum::RaceEnum;
use crate::game_card_support::entity::game_card_support_effect::GameCardSupportEffect;
use crate::game_card_support::handler::game_card_support_handler::GameCardSupportHandler;
use crate::game_card_support::service::request::use_support_card_request::UseSupportCardRequest;

pub struct SupportCard_2_Function;

impl GameCardSupportHandler for SupportCard_2_Function {
    unsafe fn use_support_card(&self, use_support_card_request: UseSupportCardRequest) -> GameCardSupportEffect {
        println!("SupportCard_2_Function: use_support_card()");

        GameCardSupportEffect::new(RaceEnum::Undead, 2)
    }
}