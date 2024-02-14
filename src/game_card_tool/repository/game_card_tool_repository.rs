use crate::game_card_tool::entity::game_card_tool_effect::GameCardToolEffect;

pub trait GameCardToolRepository {
    unsafe fn call_tool_card_repository_handler(&self, tool_card_id: i32) -> GameCardToolEffect;
}