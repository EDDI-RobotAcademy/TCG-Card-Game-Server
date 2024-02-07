use crate::common::card_attributes::card_race::card_race_enum::RaceEnum;
use crate::game_card_support::entity::game_card_support_effect::GameCardSupportEffect;
use crate::game_card_support::handler::game_card_support_handler::GameCardSupportHandler;
use crate::game_card_support::service::request::use_support_card_request::UseSupportCardRequest;

pub struct SupportCard_2_Function;

impl GameCardSupportHandler for SupportCard_2_Function {
    unsafe fn use_support_card(&self, use_support_card_request: UseSupportCardRequest) -> GameCardSupportEffect {
        println!("SupportCard_2_Function: use_support_card()");

        // TODO: 사실 모든 것이 에너지 검색이 아니므로 이 부분 전부 setter로 수정이 필요함
        let mut game_card_support_effect = GameCardSupportEffect::new(RaceEnum::Undead, 2);
        game_card_support_effect.set_need_to_find_card_id(93);

        return game_card_support_effect;
    }

    unsafe fn use_specific_support_card(&self) -> GameCardSupportEffect {
        println!("SupportCard_2_Function: use_specific_support_card()");

        // TODO: 사실 모든 것이 에너지 검색이 아니므로 이 부분 전부 setter로 수정이 필요함
        let mut game_card_support_effect = GameCardSupportEffect::new(RaceEnum::Undead, 2);
        game_card_support_effect.set_need_to_find_card_id(93);

        return game_card_support_effect;
    }
}