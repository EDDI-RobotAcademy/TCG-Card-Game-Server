use crate::common::card_attributes::card_race::card_race_enum::RaceEnum;
use crate::game_card_tool::entity::game_card_tool_effect::GameCardToolEffect;
use crate::game_card_tool::handler::game_card_tool_handler::GameCardToolHandler;

pub struct ToolCard_15_Function;

impl GameCardToolHandler for ToolCard_15_Function {
    unsafe fn summary_tool_card(&self) -> GameCardToolEffect {
        println!("ToolCard_15_Function: summary_tool_card()");

        let mut game_card_tool_effect = GameCardToolEffect::new(RaceEnum::Human, 2);
        game_card_tool_effect.set_enhance_attack_point(5);

        game_card_tool_effect
    }
}