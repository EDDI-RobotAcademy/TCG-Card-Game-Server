use std::collections::HashMap;
use std::sync::Arc;
use lazy_static::lazy_static;

use tokio::sync::Mutex as AsyncMutex;
use crate::common::card_attributes::card_grade::card_grade_enum::GradeEnum;
use crate::common::card_attributes::card_race::card_race_enum::RaceEnum;

use crate::game_card_unit::entity::game_card_unit_info::GameCardUnitInfo;
use crate::game_card_unit::handler::game_card_unit_handler::GameCardUnitHandler;
use crate::game_card_unit::handler::handler_of_19::game_card_unit_19_handler_impl::UnitCard_19_Function;
use crate::game_card_unit::handler::handler_of_26::game_card_unit_26_handler_impl::UnitCard_26_Function;
use crate::game_card_unit::handler::handler_of_27::game_card_unit_27_handler_impl::UnitCard_27_Function;
use crate::game_card_unit::handler::handler_of_31::game_card_unit_31_handler_impl::UnitCard_31_Function;
use crate::game_card_unit::handler::handler_of_32::game_card_unit_32_handler_impl::UnitCard_32_Function;
use crate::game_card_unit::handler::handler_of_6::game_card_unit_6_handler_impl::UnitCard_6_Function;
use crate::game_card_unit::repository::game_card_unit_repository::GameCardUnitRepository;

pub struct GameCardUnitRepositoryImpl {
    unit_card_functions: HashMap<i32, Box<dyn GameCardUnitHandler>>,
}

struct NoneUnitFunction;

impl GameCardUnitHandler for NoneUnitFunction {
    unsafe fn summary_unit_card(&self) -> GameCardUnitInfo {
        println!("아직 구현되지 않은 기능입니다.");

        GameCardUnitInfo::new(
            RaceEnum::Dummy,
            GradeEnum::Dummy,
            -1,
            -1,
            -1,
            -1,
            -1,
            -1,
            false,
            false,
            false,
            -1)
    }

    unsafe fn summary_unit_card_passive_default(&self) -> Vec<bool> {
        vec![false, false, false]
    }
}

impl GameCardUnitRepositoryImpl {
    fn new() -> Self {
        let mut unit_card_functions = HashMap::new();
        unit_card_functions.insert(6, Box::new(UnitCard_6_Function) as Box<dyn GameCardUnitHandler>);
        unit_card_functions.insert(11, Box::new(NoneUnitFunction) as Box<dyn GameCardUnitHandler>);
        unit_card_functions.insert(17, Box::new(NoneUnitFunction) as Box<dyn GameCardUnitHandler>);
        unit_card_functions.insert(18, Box::new(NoneUnitFunction) as Box<dyn GameCardUnitHandler>);
        unit_card_functions.insert(19, Box::new(UnitCard_19_Function) as Box<dyn GameCardUnitHandler>);
        unit_card_functions.insert(22, Box::new(NoneUnitFunction) as Box<dyn GameCardUnitHandler>);
        unit_card_functions.insert(23, Box::new(NoneUnitFunction) as Box<dyn GameCardUnitHandler>);
        unit_card_functions.insert(26, Box::new(UnitCard_26_Function) as Box<dyn GameCardUnitHandler>);
        unit_card_functions.insert(27, Box::new(UnitCard_27_Function) as Box<dyn GameCardUnitHandler>);
        unit_card_functions.insert(31, Box::new(UnitCard_31_Function) as Box<dyn GameCardUnitHandler>);
        unit_card_functions.insert(32, Box::new(UnitCard_32_Function) as Box<dyn GameCardUnitHandler>);
        unit_card_functions.insert(34, Box::new(NoneUnitFunction) as Box<dyn GameCardUnitHandler>);
        unit_card_functions.insert(38, Box::new(NoneUnitFunction) as Box<dyn GameCardUnitHandler>);
        unit_card_functions.insert(39, Box::new(NoneUnitFunction) as Box<dyn GameCardUnitHandler>);
        unit_card_functions.insert(40, Box::new(NoneUnitFunction) as Box<dyn GameCardUnitHandler>);
        unit_card_functions.insert(43, Box::new(NoneUnitFunction) as Box<dyn GameCardUnitHandler>);
        unit_card_functions.insert(44, Box::new(NoneUnitFunction) as Box<dyn GameCardUnitHandler>);
        unit_card_functions.insert(46, Box::new(NoneUnitFunction) as Box<dyn GameCardUnitHandler>);
        unit_card_functions.insert(48, Box::new(NoneUnitFunction) as Box<dyn GameCardUnitHandler>);
        unit_card_functions.insert(50, Box::new(NoneUnitFunction) as Box<dyn GameCardUnitHandler>);
        unit_card_functions.insert(51, Box::new(NoneUnitFunction) as Box<dyn GameCardUnitHandler>);
        unit_card_functions.insert(52, Box::new(NoneUnitFunction) as Box<dyn GameCardUnitHandler>);
        unit_card_functions.insert(53, Box::new(NoneUnitFunction) as Box<dyn GameCardUnitHandler>);
        unit_card_functions.insert(55, Box::new(NoneUnitFunction) as Box<dyn GameCardUnitHandler>);
        unit_card_functions.insert(56, Box::new(NoneUnitFunction) as Box<dyn GameCardUnitHandler>);
        unit_card_functions.insert(57, Box::new(NoneUnitFunction) as Box<dyn GameCardUnitHandler>);
        unit_card_functions.insert(58, Box::new(NoneUnitFunction) as Box<dyn GameCardUnitHandler>);
        unit_card_functions.insert(59, Box::new(NoneUnitFunction) as Box<dyn GameCardUnitHandler>);
        unit_card_functions.insert(60, Box::new(NoneUnitFunction) as Box<dyn GameCardUnitHandler>);
        unit_card_functions.insert(61, Box::new(NoneUnitFunction) as Box<dyn GameCardUnitHandler>);
        unit_card_functions.insert(63, Box::new(NoneUnitFunction) as Box<dyn GameCardUnitHandler>);
        unit_card_functions.insert(64, Box::new(NoneUnitFunction) as Box<dyn GameCardUnitHandler>);
        unit_card_functions.insert(70, Box::new(NoneUnitFunction) as Box<dyn GameCardUnitHandler>);
        unit_card_functions.insert(71, Box::new(NoneUnitFunction) as Box<dyn GameCardUnitHandler>);
        unit_card_functions.insert(72, Box::new(NoneUnitFunction) as Box<dyn GameCardUnitHandler>);
        unit_card_functions.insert(74, Box::new(NoneUnitFunction) as Box<dyn GameCardUnitHandler>);
        unit_card_functions.insert(75, Box::new(NoneUnitFunction) as Box<dyn GameCardUnitHandler>);
        unit_card_functions.insert(76, Box::new(NoneUnitFunction) as Box<dyn GameCardUnitHandler>);
        unit_card_functions.insert(78, Box::new(NoneUnitFunction) as Box<dyn GameCardUnitHandler>);
        unit_card_functions.insert(79, Box::new(NoneUnitFunction) as Box<dyn GameCardUnitHandler>);
        unit_card_functions.insert(80, Box::new(NoneUnitFunction) as Box<dyn GameCardUnitHandler>);
        unit_card_functions.insert(81, Box::new(NoneUnitFunction) as Box<dyn GameCardUnitHandler>);
        unit_card_functions.insert(82, Box::new(NoneUnitFunction) as Box<dyn GameCardUnitHandler>);
        unit_card_functions.insert(83, Box::new(NoneUnitFunction) as Box<dyn GameCardUnitHandler>);
        unit_card_functions.insert(84, Box::new(NoneUnitFunction) as Box<dyn GameCardUnitHandler>);
        unit_card_functions.insert(92, Box::new(NoneUnitFunction) as Box<dyn GameCardUnitHandler>);
        unit_card_functions.insert(112, Box::new(NoneUnitFunction) as Box<dyn GameCardUnitHandler>);
        unit_card_functions.insert(117, Box::new(NoneUnitFunction) as Box<dyn GameCardUnitHandler>);
        unit_card_functions.insert(120, Box::new(NoneUnitFunction) as Box<dyn GameCardUnitHandler>);
        unit_card_functions.insert(121, Box::new(NoneUnitFunction) as Box<dyn GameCardUnitHandler>);
        unit_card_functions.insert(127, Box::new(NoneUnitFunction) as Box<dyn GameCardUnitHandler>);
        unit_card_functions.insert(128, Box::new(NoneUnitFunction) as Box<dyn GameCardUnitHandler>);
        unit_card_functions.insert(130, Box::new(NoneUnitFunction) as Box<dyn GameCardUnitHandler>);
        unit_card_functions.insert(133, Box::new(NoneUnitFunction) as Box<dyn GameCardUnitHandler>);
        unit_card_functions.insert(134, Box::new(NoneUnitFunction) as Box<dyn GameCardUnitHandler>);
        unit_card_functions.insert(136, Box::new(NoneUnitFunction) as Box<dyn GameCardUnitHandler>);
        unit_card_functions.insert(139, Box::new(NoneUnitFunction) as Box<dyn GameCardUnitHandler>);
        unit_card_functions.insert(140, Box::new(NoneUnitFunction) as Box<dyn GameCardUnitHandler>);
        unit_card_functions.insert(160, Box::new(NoneUnitFunction) as Box<dyn GameCardUnitHandler>);
        unit_card_functions.insert(161, Box::new(NoneUnitFunction) as Box<dyn GameCardUnitHandler>);
        unit_card_functions.insert(162, Box::new(NoneUnitFunction) as Box<dyn GameCardUnitHandler>);
        unit_card_functions.insert(164, Box::new(NoneUnitFunction) as Box<dyn GameCardUnitHandler>);
        unit_card_functions.insert(171, Box::new(NoneUnitFunction) as Box<dyn GameCardUnitHandler>);
        unit_card_functions.insert(173, Box::new(NoneUnitFunction) as Box<dyn GameCardUnitHandler>);
        unit_card_functions.insert(174, Box::new(NoneUnitFunction) as Box<dyn GameCardUnitHandler>);
        unit_card_functions.insert(176, Box::new(NoneUnitFunction) as Box<dyn GameCardUnitHandler>);
        unit_card_functions.insert(177, Box::new(NoneUnitFunction) as Box<dyn GameCardUnitHandler>);
        unit_card_functions.insert(178, Box::new(NoneUnitFunction) as Box<dyn GameCardUnitHandler>);
        unit_card_functions.insert(179, Box::new(NoneUnitFunction) as Box<dyn GameCardUnitHandler>);
        unit_card_functions.insert(183, Box::new(NoneUnitFunction) as Box<dyn GameCardUnitHandler>);
        unit_card_functions.insert(184, Box::new(NoneUnitFunction) as Box<dyn GameCardUnitHandler>);
        unit_card_functions.insert(187, Box::new(NoneUnitFunction) as Box<dyn GameCardUnitHandler>);
        unit_card_functions.insert(189, Box::new(NoneUnitFunction) as Box<dyn GameCardUnitHandler>);
        unit_card_functions.insert(191, Box::new(NoneUnitFunction) as Box<dyn GameCardUnitHandler>);
        unit_card_functions.insert(193, Box::new(NoneUnitFunction) as Box<dyn GameCardUnitHandler>);
        unit_card_functions.insert(198, Box::new(NoneUnitFunction) as Box<dyn GameCardUnitHandler>);
        unit_card_functions.insert(200, Box::new(NoneUnitFunction) as Box<dyn GameCardUnitHandler>);
        unit_card_functions.insert(201, Box::new(NoneUnitFunction) as Box<dyn GameCardUnitHandler>);
        unit_card_functions.insert(202, Box::new(NoneUnitFunction) as Box<dyn GameCardUnitHandler>);

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
    unsafe fn call_unit_card_repository_handler(&self, unit_card_id: i32) -> GameCardUnitInfo {
        println!("GameCardUnitRepositoryImpl: call_unit_card_repository_handler()");

        let unit_card_summary_handler = self.unit_card_functions.get(&unit_card_id);
        unit_card_summary_handler.unwrap().summary_unit_card()
    }
    unsafe fn call_unit_card_passive_default_repository_handler(&self, unit_card_id: i32) -> Vec<bool> {

        let unit_card_summary_handler = self.unit_card_functions.get(&unit_card_id);
        unit_card_summary_handler.unwrap().summary_unit_card_passive_default()
    }
}

#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn test_game_card_support_repository_impl() {
        let repository = GameCardUnitRepositoryImpl::new();

        let unit_card_id = 6;
        let function1 = repository.get_function(unit_card_id);
        assert!(function1.is_some());

        let response = unsafe { function1.unwrap().summary_unit_card() };
        assert_eq!(response.get_race(), RaceEnum::Human);

        println!("6번 유닛: {:?}", response)
    }
}