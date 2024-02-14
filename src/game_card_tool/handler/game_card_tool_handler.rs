use crate::game_card_tool::entity::game_card_tool_effect::GameCardToolEffect;

pub trait GameCardToolHandler: Send {
    unsafe fn summary_tool_card(&self) -> GameCardToolEffect;
}