use crate::common::card_attributes::card_race::card_race_enum::RaceEnum;
use crate::game_card_support::entity::game_card_support_effect::GameCardSupportEffect;
use crate::game_card_support::handler::game_card_support_handler::GameCardSupportHandler;

pub struct SupportCard_2_Function;

impl GameCardSupportHandler for SupportCard_2_Function {
    unsafe fn generate_support_card_effect_summary(&self) -> GameCardSupportEffect {
        println!("SupportCard_2_Function: use_specific_support_card()");

        let mut game_card_support_effect = GameCardSupportEffect::new(RaceEnum::Undead, 2);
        game_card_support_effect.set_need_to_find_card_id(93);

        return game_card_support_effect;
    }
}