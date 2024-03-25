use crate::common::card_attributes::card_grade::card_grade_enum::GradeEnum;
use crate::common::card_attributes::card_race::card_race_enum::RaceEnum;
use crate::game_card_item::entity::game_card_item_effect::GameCardItemEffect;
use crate::game_card_item::handler::game_card_item_handler::GameCardItemHandler;
pub struct ItemCard_36_Function;

impl GameCardItemHandler for ItemCard_36_Function {
    unsafe fn summary_item_card(&self) -> GameCardItemEffect {
        println!("ItemCard_36_Function: summary_item_card()");

        let mut game_card_item_effect = GameCardItemEffect::new(RaceEnum::Undead, 0, 0, GradeEnum::Legend);
        game_card_item_effect.set_removal_amount_of_opponent_field_energy(2);

        game_card_item_effect
    }
}