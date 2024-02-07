use std::collections::HashMap;
use std::sync::Arc;
use lazy_static::lazy_static;

use tokio::sync::Mutex as AsyncMutex;
use crate::common::card_attributes::card_grade::card_grade_enum::GradeEnum;
use crate::common::card_attributes::card_race::card_race_enum::RaceEnum;

use crate::game_card_unit::entity::game_card_unit_effect::GameCardUnitEffect;
use crate::game_card_unit::handler::game_card_unit_handler::GameCardUnitHandler;
use crate::game_card_unit::handler::handler_of_6::game_card_unit_6_handler_impl::UnitCard_6_Function;
use crate::game_card_unit::repository::game_card_unit_repository::GameCardUnitRepository;

pub struct GameCardUnitRepositoryImpl {
    unit_card_functions: HashMap<i32, Box<dyn GameCardUnitHandler>>,
}

struct NoneUnitFunction;

impl GameCardUnitHandler for NoneUnitFunction {
    unsafe fn use_specific_unit_card(&self) -> GameCardUnitEffect {
        println!("아직 구현되지 않은 기능입니다.");

        GameCardUnitEffect::new(
            RaceEnum::Dummy,
            GradeEnum::Dummy,
            -1,
            -1,
            -1,
            -1,
            -1,
            -1,
            -1,
            -1,
            -1,
            -1)
    }
}

impl GameCardUnitRepositoryImpl {
    fn new() -> Self {
        let mut unit_card_functions = HashMap::new();
        unit_card_functions.insert(6, Box::new(UnitCard_6_Function) as Box<dyn GameCardUnitHandler>);
        unit_card_functions.insert(11, Box::new(NoneUnitFunction) as Box<dyn GameCardUnitHandler>);

        GameCardUnitRepositoryImpl { unit_card_functions }
    }

    fn get_function(&self, number: i32) -> Option<&Box<dyn GameCardUnitHandler>> {
        self.unit_card_functions.get(&number)
    }

    pub fn get_instance() -> Arc<AsyncMutex<GameCardUnitRepositoryImpl>> {
        lazy_static! {
            static ref INSTANCE: Arc<AsyncMutex<GameCardUnitRepositoryImpl>> =
                Arc::new(
                    AsyncMutex::new(
                        GameCardUnitRepositoryImpl::new()));
        }
        INSTANCE.clone()
    }
}

impl GameCardUnitRepository for GameCardUnitRepositoryImpl {
    unsafe fn call_unit_card_repository_handler(&self, unit_card_id: i32) -> GameCardUnitEffect {
        println!("GameCardUnitRepositoryImpl: call_unit_card_repository_handler()");

        let support_card_execution_handler = self.unit_card_functions.get(&unit_card_id);
        support_card_execution_handler.unwrap().use_specific_unit_card()
    }
}

#[cfg(test)]
mod tests {
    use std::io;
    use std::io::Write;
    use crate::common::card_attributes::card_race::card_race_enum::RaceEnum::{Human, Undead};
    use crate::game_card_support::repository::game_card_support_repository_impl::GameCardSupportRepositoryImpl;
    use super::*;
    use crate::game_card_support::service::request::use_support_card_request::UseSupportCardRequest;

    #[test]
    fn test_game_card_support_repository_impl() {
        let repository = GameCardUnitRepositoryImpl::new();

        let unit_card_id = 6;
        let function1 = repository.get_function(unit_card_id);
        assert!(function1.is_some());

        let response = unsafe { function1.unwrap().use_specific_unit_card() };
        assert_eq!(response.race(), RaceEnum::Human);

        println!("6번 유닛: {:?}", response)
    }
}