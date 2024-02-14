use crate::common::card_attributes::card_grade::card_grade_enum::GradeEnum;
use crate::common::card_attributes::card_race::card_race_enum::RaceEnum;
use crate::game_card_item::entity::game_card_item_effect::GameCardItemEffect;
use crate::game_card_item::handler::game_card_item_handler::GameCardItemHandler;

pub struct ItemCard_25_Function;

impl GameCardItemHandler for ItemCard_25_Function {
    unsafe fn summary_item_card(&self) -> GameCardItemEffect {
        println!("ItemCard_25_Function: summary_item_card()");

        let mut game_card_item_effect = GameCardItemEffect::new(RaceEnum::Undead, 0, 0, GradeEnum::Legend);
        game_card_item_effect.set_catastrophic_damage_for_field_unit(10);
        game_card_item_effect.set_catastrophic_damage_for_main_character(10);
        game_card_item_effect.set_will_be_lost_deck_card_count(1);

        game_card_item_effect
    }
}