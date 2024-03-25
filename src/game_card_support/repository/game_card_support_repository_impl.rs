use std::collections::HashMap;
use std::sync::Arc;
use lazy_static::lazy_static;

use tokio::sync::Mutex as AsyncMutex;

use crate::common::card_attributes::card_race::card_race_enum::RaceEnum::Dummy;
use crate::game_card_support::entity::game_card_support_effect::GameCardSupportEffect;
use crate::game_card_support::handler::game_card_support_handler::GameCardSupportHandler;

use crate::game_card_support::handler::handler_of_20::game_card_support_20_handler_impl::SupportCard_20_Function;
use crate::game_card_support::handler::handler_of_2::game_card_support_2_handler_impl::SupportCard_2_Function;
use crate::game_card_support::handler::handler_of_30::game_card_support_30_handler_impl::SupportCard_30_Function;

use crate::game_card_support::repository::game_card_support_repository::GameCardSupportRepository;


pub struct GameCardSupportRepositoryImpl {
    support_card_functions: HashMap<i32, Box<dyn GameCardSupportHandler>>,
}

struct NoneFunction;

impl GameCardSupportHandler for NoneFunction {
    unsafe fn generate_support_card_effect_summary(&self) -> GameCardSupportEffect {
        println!("아직 구현되지 않은 기능입니다.");

        GameCardSupportEffect::new(Dummy, 0)
    }
}

impl GameCardSupportRepositoryImpl {
    fn new() -> Self {
        let mut support_card_functions = HashMap::new();
        support_card_functions.insert(2, Box::new(SupportCard_2_Function) as Box<dyn GameCardSupportHandler>);
        support_card_functions.insert(5, Box::new(NoneFunction) as Box<dyn GameCardSupportHandler>);
        support_card_functions.insert(7, Box::new(NoneFunction) as Box<dyn GameCardSupportHandler>);
        support_card_functions.insert(10, Box::new(NoneFunction) as Box<dyn GameCardSupportHandler>);
        support_card_functions.insert(16, Box::new(NoneFunction) as Box<dyn GameCardSupportHandler>);
        support_card_functions.insert(20, Box::new(SupportCard_20_Function) as Box<dyn GameCardSupportHandler>);
        support_card_functions.insert(21, Box::new(NoneFunction) as Box<dyn GameCardSupportHandler>);
        support_card_functions.insert(24, Box::new(NoneFunction) as Box<dyn GameCardSupportHandler>);
        support_card_functions.insert(28, Box::new(NoneFunction) as Box<dyn GameCardSupportHandler>);
        support_card_functions.insert(29, Box::new(NoneFunction) as Box<dyn GameCardSupportHandler>);
        support_card_functions.insert(30, Box::new(SupportCard_30_Function) as Box<dyn GameCardSupportHandler>);
        support_card_functions.insert(41, Box::new(NoneFunction) as Box<dyn GameCardSupportHandler>);
        support_card_functions.insert(47, Box::new(NoneFunction) as Box<dyn GameCardSupportHandler>);
        support_card_functions.insert(65, Box::new(NoneFunction) as Box<dyn GameCardSupportHandler>);
        support_card_functions.insert(69, Box::new(NoneFunction) as Box<dyn GameCardSupportHandler>);
        support_card_functions.insert(77, Box::new(NoneFunction) as Box<dyn GameCardSupportHandler>);
        support_card_functions.insert(87, Box::new(NoneFunction) as Box<dyn GameCardSupportHandler>);
        support_card_functions.insert(94, Box::new(NoneFunction) as Box<dyn GameCardSupportHandler>);
        support_card_functions.insert(116, Box::new(NoneFunction) as Box<dyn GameCardSupportHandler>);
        support_card_functions.insert(126, Box::new(NoneFunction) as Box<dyn GameCardSupportHandler>);
        support_card_functions.insert(143, Box::new(NoneFunction) as Box<dyn GameCardSupportHandler>);
        support_card_functions.insert(144, Box::new(NoneFunction) as Box<dyn GameCardSupportHandler>);
        support_card_functions.insert(146, Box::new(NoneFunction) as Box<dyn GameCardSupportHandler>);
        support_card_functions.insert(156, Box::new(NoneFunction) as Box<dyn GameCardSupportHandler>);
        support_card_functions.insert(163, Box::new(NoneFunction) as Box<dyn GameCardSupportHandler>);
        support_card_functions.insert(165, Box::new(NoneFunction) as Box<dyn GameCardSupportHandler>);
        support_card_functions.insert(166, Box::new(NoneFunction) as Box<dyn GameCardSupportHandler>);
        support_card_functions.insert(167, Box::new(NoneFunction) as Box<dyn GameCardSupportHandler>);
        support_card_functions.insert(169, Box::new(NoneFunction) as Box<dyn GameCardSupportHandler>);
        support_card_functions.insert(170, Box::new(NoneFunction) as Box<dyn GameCardSupportHandler>);
        support_card_functions.insert(172, Box::new(NoneFunction) as Box<dyn GameCardSupportHandler>);
        support_card_functions.insert(175, Box::new(NoneFunction) as Box<dyn GameCardSupportHandler>);
        support_card_functions.insert(186, Box::new(NoneFunction) as Box<dyn GameCardSupportHandler>);
        support_card_functions.insert(188, Box::new(NoneFunction) as Box<dyn GameCardSupportHandler>);
        support_card_functions.insert(190, Box::new(NoneFunction) as Box<dyn GameCardSupportHandler>);
        support_card_functions.insert(195, Box::new(NoneFunction) as Box<dyn GameCardSupportHandler>);

        GameCardSupportRepositoryImpl { support_card_functions }
    }

    fn get_function(&self, number: i32) -> Option<&Box<dyn GameCardSupportHandler>> {
        self.support_card_functions.get(&number)
    }

    pub fn get_instance() -> Arc<AsyncMutex<GameCardSupportRepositoryImpl>> {
        lazy_static! {
            static ref INSTANCE: Arc<AsyncMutex<GameCardSupportRepositoryImpl>> =
                Arc::new(
                    AsyncMutex::new(
                        GameCardSupportRepositoryImpl::new()));
        }
        INSTANCE.clone()
    }
}

impl GameCardSupportRepository for GameCardSupportRepositoryImpl {
    unsafe fn call_support_card_repository_handler(&self, support_card_id: i32) -> GameCardSupportEffect {
        println!("GameCardSupportRepositoryImpl: call_support_card_repository_handler()");

        let support_card_execution_handler = self.support_card_functions.get(&support_card_id);
        support_card_execution_handler.unwrap().generate_support_card_effect_summary()
    }
}