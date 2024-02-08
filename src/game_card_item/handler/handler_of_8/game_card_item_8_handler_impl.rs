use crate::common::card_attributes::card_grade::card_grade_enum::GradeEnum;
use crate::common::card_attributes::card_race::card_race_enum::RaceEnum;
use crate::game_card_item::entity::game_card_item_effect::GameCardItemEffect;
use crate::game_card_item::handler::game_card_item_handler::GameCardItemHandler;
use crate::game_card_item::service::request::use_item_card_request::UseItemCardRequest;
use crate::game_card_item::service::response::use_item_card_response::UseItemCardResponse;

pub struct ItemCard_8_Function;

impl GameCardItemHandler for ItemCard_8_Function {
    unsafe fn use_item_card(&self) -> GameCardItemEffect {
        println!("ItemCard_8_Function: use_item_card()");

        GameCardItemEffect::new(RaceEnum::Undead, 2, 30, GradeEnum::Legend)
    }
}