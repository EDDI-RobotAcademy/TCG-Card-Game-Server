use crate::common::card_attributes::card_grade::card_grade_enum::GradeEnum;
use crate::common::card_attributes::card_race::card_race_enum::RaceEnum;
use crate::game_card_item::entity::game_card_item_effect::GameCardItemEffect;
use crate::game_card_item::handler::game_card_item_handler::GameCardItemHandler;

pub struct ItemCard_33_Function;

impl GameCardItemHandler for ItemCard_33_Function {
    unsafe fn summary_item_card(&self) -> GameCardItemEffect {
        println!("ItemCard_33_Function: summary_item_card()");

        let mut game_card_item_effect =
            GameCardItemEffect::new(RaceEnum::Undead,
                                    0,
                                    0,
                                    GradeEnum::Legend);

        let mut will_be_sacrificed_unit_list = Vec::new();

        will_be_sacrificed_unit_list.push(31);
        will_be_sacrificed_unit_list.push(32);

        game_card_item_effect.set_target_count_that_can_be_damaged(2);
        game_card_item_effect.set_unit_list_that_can_be_sacrificed(will_be_sacrificed_unit_list);

        game_card_item_effect
    }
}