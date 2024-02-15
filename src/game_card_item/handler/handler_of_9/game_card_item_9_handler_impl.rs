use crate::common::card_attributes::card_grade::card_grade_enum::GradeEnum;
use crate::common::card_attributes::card_race::card_race_enum::RaceEnum;
use crate::game_card_item::entity::game_card_item_effect::GameCardItemEffect;
use crate::game_card_item::handler::game_card_item_handler::GameCardItemHandler;

pub struct ItemCard_9_Function;

impl GameCardItemHandler for ItemCard_9_Function {
    unsafe fn summary_item_card(&self) -> GameCardItemEffect {
        println!("ItemCard_9_Function: summary_item_card()");

        let mut game_card_item_effect =
            GameCardItemEffect::new(RaceEnum::Undead,
                                    0,
                                    10,
                                    GradeEnum::Legend);

        game_card_item_effect.set_will_be_removed_energy_count(2);

        game_card_item_effect
    }
}