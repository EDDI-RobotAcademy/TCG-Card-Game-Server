use crate::common::card_attributes::card_race::card_race_enum::RaceEnum;
use crate::game_card_support::entity::game_card_support_effect::GameCardSupportEffect;
use crate::game_card_support::handler::game_card_support_handler::GameCardSupportHandler;
use crate::game_card_support::service::request::use_support_card_request::UseSupportCardRequest;

pub struct SupportCard_20_Function;

impl GameCardSupportHandler for SupportCard_20_Function {
    unsafe fn use_support_card(&self, use_support_card_request: UseSupportCardRequest) -> GameCardSupportEffect {
        todo!()
    }

    unsafe fn use_specific_support_card(&self) -> GameCardSupportEffect {
        let mut game_card_support_effect = GameCardSupportEffect::new(RaceEnum::Undead, 0);
        game_card_support_effect.set_need_to_draw_card_count(2);

        game_card_support_effect
    }
}