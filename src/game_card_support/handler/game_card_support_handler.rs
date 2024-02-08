use std::println;
use crate::common::card_attributes::card_race::card_race_enum::RaceEnum;
use crate::game_card_support::entity::game_card_support_effect::GameCardSupportEffect;
use crate::game_card_support::service::request::use_support_card_request::UseSupportCardRequest;

pub trait GameCardSupportHandler: Send {
    unsafe fn use_support_card(&self, use_support_card_request: UseSupportCardRequest) -> GameCardSupportEffect;
    unsafe fn use_specific_support_card(&self) -> GameCardSupportEffect;
}