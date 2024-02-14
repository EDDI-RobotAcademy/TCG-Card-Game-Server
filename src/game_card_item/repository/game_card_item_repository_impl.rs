use std::collections::HashMap;
use std::sync::Arc;
use lazy_static::lazy_static;

use tokio::sync::Mutex as AsyncMutex;

use crate::common::card_attributes::card_grade::card_grade_enum::GradeEnum;
use crate::common::card_attributes::card_race::card_race_enum::RaceEnum;
use crate::game_card_item::entity::game_card_item_effect::GameCardItemEffect;
use crate::game_card_item::handler::game_card_item_handler::GameCardItemHandler;
use crate::game_card_item::handler::handler_of_25::game_card_item_25_handler_impl::ItemCard_25_Function;
use crate::game_card_item::handler::handler_of_33::game_card_item_33_handler_impl::ItemCard_33_Function;
use crate::game_card_item::handler::handler_of_35::game_card_item_35_handler_impl::ItemCard_35_Function;
use crate::game_card_item::handler::handler_of_8::game_card_item_8_handler_impl::ItemCard_8_Function;
use crate::game_card_item::repository::game_card_item_repository::GameCardItemRepository;

pub struct GameCardItemRepositoryImpl {
    item_card_functions: HashMap<i32, Box<dyn GameCardItemHandler>>,
}

struct NoneFunction;

impl GameCardItemHandler for NoneFunction {
    unsafe fn summary_item_card(&self) -> GameCardItemEffect {
        println!("아직 구현되지 않은 기능입니다.");

        GameCardItemEffect::new(
            RaceEnum::Dummy,
            -1,
            -1,
            GradeEnum::Dummy)
    }
}

impl GameCardItemRepositoryImpl {
    fn new() -> Self {
        let mut item_card_functions = HashMap::new();
        item_card_functions.insert(8, Box::new(ItemCard_8_Function) as Box<dyn GameCardItemHandler>);
        item_card_functions.insert(9, Box::new(NoneFunction) as Box<dyn GameCardItemHandler>);
        item_card_functions.insert(13, Box::new(NoneFunction) as Box<dyn GameCardItemHandler>);
        item_card_functions.insert(25, Box::new(ItemCard_25_Function) as Box<dyn GameCardItemHandler>);
        item_card_functions.insert(30, Box::new(NoneFunction) as Box<dyn GameCardItemHandler>);
        item_card_functions.insert(33, Box::new(ItemCard_33_Function) as Box<dyn GameCardItemHandler>);
        item_card_functions.insert(35, Box::new(ItemCard_35_Function) as Box<dyn GameCardItemHandler>);
        item_card_functions.insert(45, Box::new(NoneFunction) as Box<dyn GameCardItemHandler>);
        item_card_functions.insert(54, Box::new(NoneFunction) as Box<dyn GameCardItemHandler>);
        item_card_functions.insert(66, Box::new(NoneFunction) as Box<dyn GameCardItemHandler>);
        item_card_functions.insert(88, Box::new(NoneFunction) as Box<dyn GameCardItemHandler>);
        item_card_functions.insert(95, Box::new(NoneFunction) as Box<dyn GameCardItemHandler>);
        item_card_functions.insert(108, Box::new(NoneFunction) as Box<dyn GameCardItemHandler>);
        item_card_functions.insert(113, Box::new(NoneFunction) as Box<dyn GameCardItemHandler>);
        item_card_functions.insert(119, Box::new(NoneFunction) as Box<dyn GameCardItemHandler>);
        item_card_functions.insert(125, Box::new(NoneFunction) as Box<dyn GameCardItemHandler>);
        item_card_functions.insert(137, Box::new(NoneFunction) as Box<dyn GameCardItemHandler>);
        item_card_functions.insert(141, Box::new(NoneFunction) as Box<dyn GameCardItemHandler>);
        item_card_functions.insert(154, Box::new(NoneFunction) as Box<dyn GameCardItemHandler>);
        item_card_functions.insert(194, Box::new(NoneFunction) as Box<dyn GameCardItemHandler>);

        GameCardItemRepositoryImpl { item_card_functions }
    }

    fn get_function(&self, number: i32) -> Option<&Box<dyn GameCardItemHandler>> {
        self.item_card_functions.get(&number)
    }

    pub fn get_instance() -> Arc<AsyncMutex<GameCardItemRepositoryImpl>> {
        lazy_static! {
            static ref INSTANCE: Arc<AsyncMutex<GameCardItemRepositoryImpl>> =
                Arc::new(
                    AsyncMutex::new(
                        GameCardItemRepositoryImpl::new()));
        }
        INSTANCE.clone()
    }
}

impl GameCardItemRepository for GameCardItemRepositoryImpl {
    unsafe fn call_item_card_repository_handler(&self, item_card_id: i32) -> GameCardItemEffect {
        println!("GameCardSupportRepositoryImpl: call_support_card_repository_table()");

        let item_card_summary_handler = self.item_card_functions.get(&item_card_id);
        item_card_summary_handler.unwrap().summary_item_card()
    }
}

#[cfg(test)]
mod tests {
    use std::io;
    use std::io::Write;
    use crate::game_card_item::service::request::use_item_card_request::UseItemCardRequest;
    use crate::game_card_item::service::response::use_item_card_response::UseItemCardResponse;
    use super::*;

    #[test]
    fn test_game_card_item_repository_impl() {
        let repository = GameCardItemRepositoryImpl::new();

        let item_number = 8;
        let function = repository.get_function(item_number);
        assert!(function.is_some());

        let response = unsafe { function.unwrap().summary_item_card() };
        assert_eq!(response.get_required_energy().get_required_energy_race(), &RaceEnum::Undead);
        println!("item effect: {:?}", response)
    }
}

