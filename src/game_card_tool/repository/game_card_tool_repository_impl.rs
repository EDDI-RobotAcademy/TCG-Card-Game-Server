use std::collections::HashMap;
use std::sync::Arc;
use lazy_static::lazy_static;

use tokio::sync::Mutex as AsyncMutex;

use crate::common::card_attributes::card_race::card_race_enum::RaceEnum::Dummy;
use crate::game_card_tool::entity::game_card_tool_effect::GameCardToolEffect;
use crate::game_card_tool::handler::game_card_tool_handler::GameCardToolHandler;

use crate::game_card_tool::handler::handler_of_15::game_card_tool_15_handler_impl::ToolCard_15_Function;

use crate::game_card_tool::repository::game_card_tool_repository::GameCardToolRepository;

pub struct GameCardToolRepositoryImpl {
    tool_card_functions: HashMap<i32, Box<dyn GameCardToolHandler>>,
}

struct NoneFunction;

impl GameCardToolHandler for NoneFunction {
    unsafe fn summary_tool_card(&self) -> GameCardToolEffect {
        println!("아직 구현되지 않은 기능입니다.");

        GameCardToolEffect::new(Dummy, 0)
    }
}

impl GameCardToolRepositoryImpl {
    fn new() -> Self {
        let mut tool_card_functions = HashMap::new();
        tool_card_functions.insert(14, Box::new(NoneFunction) as Box<dyn GameCardToolHandler>);
        tool_card_functions.insert(15, Box::new(ToolCard_15_Function) as Box<dyn GameCardToolHandler>);
        tool_card_functions.insert(42, Box::new(NoneFunction) as Box<dyn GameCardToolHandler>);
        tool_card_functions.insert(49, Box::new(NoneFunction) as Box<dyn GameCardToolHandler>);
        tool_card_functions.insert(62, Box::new(NoneFunction) as Box<dyn GameCardToolHandler>);
        tool_card_functions.insert(129, Box::new(NoneFunction) as Box<dyn GameCardToolHandler>);
        tool_card_functions.insert(155, Box::new(NoneFunction) as Box<dyn GameCardToolHandler>);

        GameCardToolRepositoryImpl { tool_card_functions }
    }

    fn get_function(&self, number: i32) -> Option<&Box<dyn GameCardToolHandler>> {
        self.tool_card_functions.get(&number)
    }

    pub fn get_instance() -> Arc<AsyncMutex<GameCardToolRepositoryImpl>> {
        lazy_static! {
            static ref INSTANCE: Arc<AsyncMutex<GameCardToolRepositoryImpl>> =
                Arc::new(
                    AsyncMutex::new(
                        GameCardToolRepositoryImpl::new()));
        }
        INSTANCE.clone()
    }
}

impl GameCardToolRepository for GameCardToolRepositoryImpl {
    unsafe fn call_tool_card_repository_handler(&self, tool_card_id: i32) -> GameCardToolEffect {
        println!("GameCardToolRepositoryImpl: call_tool_card_repository_handler()");

        let tool_card_execution_handler = self.tool_card_functions.get(&tool_card_id);
        tool_card_execution_handler.unwrap().summary_tool_card()
    }
}