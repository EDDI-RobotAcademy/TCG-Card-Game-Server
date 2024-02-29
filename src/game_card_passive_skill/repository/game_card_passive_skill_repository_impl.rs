use std::collections::HashMap;
use std::sync::Arc;
use lazy_static::lazy_static;

use tokio::sync::Mutex as AsyncMutex;
use crate::game_card_passive_skill::entity::passive_skill_casting_condition::PassiveSkillCastingCondition;

use crate::game_card_passive_skill::entity::passive_skill_type::PassiveSkillType;
use crate::game_card_passive_skill::entity::summary_passive_skill_effect::SummaryPassiveSkillEffect;
use crate::game_card_passive_skill::handler::game_card_passive_skill_handler::GameCardPassiveSkillHandler;
use crate::game_card_passive_skill::handler::slot::first_slot::handler_of_19::game_card_unit_19_passive_slot_1_handler_impl::UnitCard_19_Passive_Slot_1_Function;
use crate::game_card_passive_skill::handler::slot::second_slot::handler_of_19::game_card_unit_19_passive_slot_2_handler_impl::UnitCard_19_Passive_Slot_2_Function;
use crate::game_card_passive_skill::repository::game_card_passive_skill_repository::GameCardPassiveSkillRepository;

pub struct GameCardPassiveSkillRepositoryImpl {
    passive_skill_functions: HashMap<i32, HashMap<i32, Box<dyn GameCardPassiveSkillHandler>>>,
}

struct NonePassiveSkillFunction;

impl GameCardPassiveSkillHandler for NonePassiveSkillFunction {
    unsafe fn summary_passive_skill(&self) -> SummaryPassiveSkillEffect {
        println!("아직 구현되지 않은 기능입니다.");

        SummaryPassiveSkillEffect::new(
            PassiveSkillType::Dummy, vec![PassiveSkillCastingCondition::Dummy], -1)
    }
}

impl GameCardPassiveSkillRepositoryImpl {
    fn new() -> Self {
        let mut passive_skill_functions = HashMap::new();

        for passive_skill_index in &[1, 2, 3] {
            let inner_map = Self::create_inner_map_for_passive_skill(*passive_skill_index);
            passive_skill_functions.insert(*passive_skill_index, inner_map);
        }

        GameCardPassiveSkillRepositoryImpl { passive_skill_functions }
    }

    fn create_inner_map_for_passive_skill(passive_skill_index: i32) -> HashMap<i32, Box<dyn GameCardPassiveSkillHandler>> {
        let mut inner_map = HashMap::new();

        match passive_skill_index {
            1 => {
                inner_map.insert(6, Box::new(NonePassiveSkillFunction) as Box<dyn GameCardPassiveSkillHandler>);
                inner_map.insert(11, Box::new(NonePassiveSkillFunction) as Box<dyn GameCardPassiveSkillHandler>);
                inner_map.insert(17, Box::new(NonePassiveSkillFunction) as Box<dyn GameCardPassiveSkillHandler>);
                inner_map.insert(18, Box::new(NonePassiveSkillFunction) as Box<dyn GameCardPassiveSkillHandler>);
                inner_map.insert(19, Box::new(UnitCard_19_Passive_Slot_1_Function) as Box<dyn GameCardPassiveSkillHandler>);
                inner_map.insert(22, Box::new(NonePassiveSkillFunction) as Box<dyn GameCardPassiveSkillHandler>);
                inner_map.insert(23, Box::new(NonePassiveSkillFunction) as Box<dyn GameCardPassiveSkillHandler>);
                inner_map.insert(26, Box::new(NonePassiveSkillFunction) as Box<dyn GameCardPassiveSkillHandler>);
                inner_map.insert(27, Box::new(NonePassiveSkillFunction) as Box<dyn GameCardPassiveSkillHandler>);
                inner_map.insert(31, Box::new(NonePassiveSkillFunction) as Box<dyn GameCardPassiveSkillHandler>);
                inner_map.insert(32, Box::new(NonePassiveSkillFunction) as Box<dyn GameCardPassiveSkillHandler>);
                inner_map.insert(34, Box::new(NonePassiveSkillFunction) as Box<dyn GameCardPassiveSkillHandler>);
                inner_map.insert(38, Box::new(NonePassiveSkillFunction) as Box<dyn GameCardPassiveSkillHandler>);
                inner_map.insert(39, Box::new(NonePassiveSkillFunction) as Box<dyn GameCardPassiveSkillHandler>);
                inner_map.insert(40, Box::new(NonePassiveSkillFunction) as Box<dyn GameCardPassiveSkillHandler>);
                inner_map.insert(43, Box::new(NonePassiveSkillFunction) as Box<dyn GameCardPassiveSkillHandler>);
                inner_map.insert(44, Box::new(NonePassiveSkillFunction) as Box<dyn GameCardPassiveSkillHandler>);
                inner_map.insert(46, Box::new(NonePassiveSkillFunction) as Box<dyn GameCardPassiveSkillHandler>);
                inner_map.insert(48, Box::new(NonePassiveSkillFunction) as Box<dyn GameCardPassiveSkillHandler>);
                inner_map.insert(50, Box::new(NonePassiveSkillFunction) as Box<dyn GameCardPassiveSkillHandler>);
                inner_map.insert(51, Box::new(NonePassiveSkillFunction) as Box<dyn GameCardPassiveSkillHandler>);
                inner_map.insert(52, Box::new(NonePassiveSkillFunction) as Box<dyn GameCardPassiveSkillHandler>);
                inner_map.insert(53, Box::new(NonePassiveSkillFunction) as Box<dyn GameCardPassiveSkillHandler>);
                inner_map.insert(55, Box::new(NonePassiveSkillFunction) as Box<dyn GameCardPassiveSkillHandler>);
                inner_map.insert(56, Box::new(NonePassiveSkillFunction) as Box<dyn GameCardPassiveSkillHandler>);
                inner_map.insert(57, Box::new(NonePassiveSkillFunction) as Box<dyn GameCardPassiveSkillHandler>);
                inner_map.insert(58, Box::new(NonePassiveSkillFunction) as Box<dyn GameCardPassiveSkillHandler>);
                inner_map.insert(59, Box::new(NonePassiveSkillFunction) as Box<dyn GameCardPassiveSkillHandler>);
                inner_map.insert(60, Box::new(NonePassiveSkillFunction) as Box<dyn GameCardPassiveSkillHandler>);
                inner_map.insert(61, Box::new(NonePassiveSkillFunction) as Box<dyn GameCardPassiveSkillHandler>);
                inner_map.insert(63, Box::new(NonePassiveSkillFunction) as Box<dyn GameCardPassiveSkillHandler>);
                inner_map.insert(64, Box::new(NonePassiveSkillFunction) as Box<dyn GameCardPassiveSkillHandler>);
                inner_map.insert(70, Box::new(NonePassiveSkillFunction) as Box<dyn GameCardPassiveSkillHandler>);
                inner_map.insert(71, Box::new(NonePassiveSkillFunction) as Box<dyn GameCardPassiveSkillHandler>);
                inner_map.insert(72, Box::new(NonePassiveSkillFunction) as Box<dyn GameCardPassiveSkillHandler>);
                inner_map.insert(74, Box::new(NonePassiveSkillFunction) as Box<dyn GameCardPassiveSkillHandler>);
                inner_map.insert(75, Box::new(NonePassiveSkillFunction) as Box<dyn GameCardPassiveSkillHandler>);
                inner_map.insert(76, Box::new(NonePassiveSkillFunction) as Box<dyn GameCardPassiveSkillHandler>);
                inner_map.insert(78, Box::new(NonePassiveSkillFunction) as Box<dyn GameCardPassiveSkillHandler>);
                inner_map.insert(79, Box::new(NonePassiveSkillFunction) as Box<dyn GameCardPassiveSkillHandler>);
                inner_map.insert(80, Box::new(NonePassiveSkillFunction) as Box<dyn GameCardPassiveSkillHandler>);
                inner_map.insert(81, Box::new(NonePassiveSkillFunction) as Box<dyn GameCardPassiveSkillHandler>);
                inner_map.insert(82, Box::new(NonePassiveSkillFunction) as Box<dyn GameCardPassiveSkillHandler>);
                inner_map.insert(83, Box::new(NonePassiveSkillFunction) as Box<dyn GameCardPassiveSkillHandler>);
                inner_map.insert(84, Box::new(NonePassiveSkillFunction) as Box<dyn GameCardPassiveSkillHandler>);
                inner_map.insert(92, Box::new(NonePassiveSkillFunction) as Box<dyn GameCardPassiveSkillHandler>);
                inner_map.insert(112, Box::new(NonePassiveSkillFunction) as Box<dyn GameCardPassiveSkillHandler>);
                inner_map.insert(117, Box::new(NonePassiveSkillFunction) as Box<dyn GameCardPassiveSkillHandler>);
                inner_map.insert(120, Box::new(NonePassiveSkillFunction) as Box<dyn GameCardPassiveSkillHandler>);
                inner_map.insert(121, Box::new(NonePassiveSkillFunction) as Box<dyn GameCardPassiveSkillHandler>);
                inner_map.insert(127, Box::new(NonePassiveSkillFunction) as Box<dyn GameCardPassiveSkillHandler>);
                inner_map.insert(128, Box::new(NonePassiveSkillFunction) as Box<dyn GameCardPassiveSkillHandler>);
                inner_map.insert(130, Box::new(NonePassiveSkillFunction) as Box<dyn GameCardPassiveSkillHandler>);
                inner_map.insert(133, Box::new(NonePassiveSkillFunction) as Box<dyn GameCardPassiveSkillHandler>);
                inner_map.insert(134, Box::new(NonePassiveSkillFunction) as Box<dyn GameCardPassiveSkillHandler>);
                inner_map.insert(136, Box::new(NonePassiveSkillFunction) as Box<dyn GameCardPassiveSkillHandler>);
                inner_map.insert(139, Box::new(NonePassiveSkillFunction) as Box<dyn GameCardPassiveSkillHandler>);
                inner_map.insert(140, Box::new(NonePassiveSkillFunction) as Box<dyn GameCardPassiveSkillHandler>);
                inner_map.insert(160, Box::new(NonePassiveSkillFunction) as Box<dyn GameCardPassiveSkillHandler>);
                inner_map.insert(161, Box::new(NonePassiveSkillFunction) as Box<dyn GameCardPassiveSkillHandler>);
                inner_map.insert(162, Box::new(NonePassiveSkillFunction) as Box<dyn GameCardPassiveSkillHandler>);
                inner_map.insert(164, Box::new(NonePassiveSkillFunction) as Box<dyn GameCardPassiveSkillHandler>);
                inner_map.insert(171, Box::new(NonePassiveSkillFunction) as Box<dyn GameCardPassiveSkillHandler>);
                inner_map.insert(173, Box::new(NonePassiveSkillFunction) as Box<dyn GameCardPassiveSkillHandler>);
                inner_map.insert(174, Box::new(NonePassiveSkillFunction) as Box<dyn GameCardPassiveSkillHandler>);
                inner_map.insert(176, Box::new(NonePassiveSkillFunction) as Box<dyn GameCardPassiveSkillHandler>);
                inner_map.insert(177, Box::new(NonePassiveSkillFunction) as Box<dyn GameCardPassiveSkillHandler>);
                inner_map.insert(178, Box::new(NonePassiveSkillFunction) as Box<dyn GameCardPassiveSkillHandler>);
                inner_map.insert(179, Box::new(NonePassiveSkillFunction) as Box<dyn GameCardPassiveSkillHandler>);
                inner_map.insert(183, Box::new(NonePassiveSkillFunction) as Box<dyn GameCardPassiveSkillHandler>);
                inner_map.insert(184, Box::new(NonePassiveSkillFunction) as Box<dyn GameCardPassiveSkillHandler>);
                inner_map.insert(187, Box::new(NonePassiveSkillFunction) as Box<dyn GameCardPassiveSkillHandler>);
                inner_map.insert(189, Box::new(NonePassiveSkillFunction) as Box<dyn GameCardPassiveSkillHandler>);
                inner_map.insert(191, Box::new(NonePassiveSkillFunction) as Box<dyn GameCardPassiveSkillHandler>);
                inner_map.insert(193, Box::new(NonePassiveSkillFunction) as Box<dyn GameCardPassiveSkillHandler>);
                inner_map.insert(198, Box::new(NonePassiveSkillFunction) as Box<dyn GameCardPassiveSkillHandler>);
                inner_map.insert(200, Box::new(NonePassiveSkillFunction) as Box<dyn GameCardPassiveSkillHandler>);
                inner_map.insert(201, Box::new(NonePassiveSkillFunction) as Box<dyn GameCardPassiveSkillHandler>);
                inner_map.insert(202, Box::new(NonePassiveSkillFunction) as Box<dyn GameCardPassiveSkillHandler>);
            }
            2 => {
                inner_map.insert(6, Box::new(NonePassiveSkillFunction) as Box<dyn GameCardPassiveSkillHandler>);
                inner_map.insert(11, Box::new(NonePassiveSkillFunction) as Box<dyn GameCardPassiveSkillHandler>);
                inner_map.insert(17, Box::new(NonePassiveSkillFunction) as Box<dyn GameCardPassiveSkillHandler>);
                inner_map.insert(18, Box::new(NonePassiveSkillFunction) as Box<dyn GameCardPassiveSkillHandler>);
                inner_map.insert(19, Box::new(UnitCard_19_Passive_Slot_2_Function) as Box<dyn GameCardPassiveSkillHandler>);
                inner_map.insert(22, Box::new(NonePassiveSkillFunction) as Box<dyn GameCardPassiveSkillHandler>);
                inner_map.insert(23, Box::new(NonePassiveSkillFunction) as Box<dyn GameCardPassiveSkillHandler>);
                inner_map.insert(26, Box::new(NonePassiveSkillFunction) as Box<dyn GameCardPassiveSkillHandler>);
                inner_map.insert(27, Box::new(NonePassiveSkillFunction) as Box<dyn GameCardPassiveSkillHandler>);
                inner_map.insert(31, Box::new(NonePassiveSkillFunction) as Box<dyn GameCardPassiveSkillHandler>);
                inner_map.insert(32, Box::new(NonePassiveSkillFunction) as Box<dyn GameCardPassiveSkillHandler>);
                inner_map.insert(34, Box::new(NonePassiveSkillFunction) as Box<dyn GameCardPassiveSkillHandler>);
                inner_map.insert(38, Box::new(NonePassiveSkillFunction) as Box<dyn GameCardPassiveSkillHandler>);
                inner_map.insert(39, Box::new(NonePassiveSkillFunction) as Box<dyn GameCardPassiveSkillHandler>);
                inner_map.insert(40, Box::new(NonePassiveSkillFunction) as Box<dyn GameCardPassiveSkillHandler>);
                inner_map.insert(43, Box::new(NonePassiveSkillFunction) as Box<dyn GameCardPassiveSkillHandler>);
                inner_map.insert(44, Box::new(NonePassiveSkillFunction) as Box<dyn GameCardPassiveSkillHandler>);
                inner_map.insert(46, Box::new(NonePassiveSkillFunction) as Box<dyn GameCardPassiveSkillHandler>);
                inner_map.insert(48, Box::new(NonePassiveSkillFunction) as Box<dyn GameCardPassiveSkillHandler>);
                inner_map.insert(50, Box::new(NonePassiveSkillFunction) as Box<dyn GameCardPassiveSkillHandler>);
                inner_map.insert(51, Box::new(NonePassiveSkillFunction) as Box<dyn GameCardPassiveSkillHandler>);
                inner_map.insert(52, Box::new(NonePassiveSkillFunction) as Box<dyn GameCardPassiveSkillHandler>);
                inner_map.insert(53, Box::new(NonePassiveSkillFunction) as Box<dyn GameCardPassiveSkillHandler>);
                inner_map.insert(55, Box::new(NonePassiveSkillFunction) as Box<dyn GameCardPassiveSkillHandler>);
                inner_map.insert(56, Box::new(NonePassiveSkillFunction) as Box<dyn GameCardPassiveSkillHandler>);
                inner_map.insert(57, Box::new(NonePassiveSkillFunction) as Box<dyn GameCardPassiveSkillHandler>);
                inner_map.insert(58, Box::new(NonePassiveSkillFunction) as Box<dyn GameCardPassiveSkillHandler>);
                inner_map.insert(59, Box::new(NonePassiveSkillFunction) as Box<dyn GameCardPassiveSkillHandler>);
                inner_map.insert(60, Box::new(NonePassiveSkillFunction) as Box<dyn GameCardPassiveSkillHandler>);
                inner_map.insert(61, Box::new(NonePassiveSkillFunction) as Box<dyn GameCardPassiveSkillHandler>);
                inner_map.insert(63, Box::new(NonePassiveSkillFunction) as Box<dyn GameCardPassiveSkillHandler>);
                inner_map.insert(64, Box::new(NonePassiveSkillFunction) as Box<dyn GameCardPassiveSkillHandler>);
                inner_map.insert(70, Box::new(NonePassiveSkillFunction) as Box<dyn GameCardPassiveSkillHandler>);
                inner_map.insert(71, Box::new(NonePassiveSkillFunction) as Box<dyn GameCardPassiveSkillHandler>);
                inner_map.insert(72, Box::new(NonePassiveSkillFunction) as Box<dyn GameCardPassiveSkillHandler>);
                inner_map.insert(74, Box::new(NonePassiveSkillFunction) as Box<dyn GameCardPassiveSkillHandler>);
                inner_map.insert(75, Box::new(NonePassiveSkillFunction) as Box<dyn GameCardPassiveSkillHandler>);
                inner_map.insert(76, Box::new(NonePassiveSkillFunction) as Box<dyn GameCardPassiveSkillHandler>);
                inner_map.insert(78, Box::new(NonePassiveSkillFunction) as Box<dyn GameCardPassiveSkillHandler>);
                inner_map.insert(79, Box::new(NonePassiveSkillFunction) as Box<dyn GameCardPassiveSkillHandler>);
                inner_map.insert(80, Box::new(NonePassiveSkillFunction) as Box<dyn GameCardPassiveSkillHandler>);
                inner_map.insert(81, Box::new(NonePassiveSkillFunction) as Box<dyn GameCardPassiveSkillHandler>);
                inner_map.insert(82, Box::new(NonePassiveSkillFunction) as Box<dyn GameCardPassiveSkillHandler>);
                inner_map.insert(83, Box::new(NonePassiveSkillFunction) as Box<dyn GameCardPassiveSkillHandler>);
                inner_map.insert(84, Box::new(NonePassiveSkillFunction) as Box<dyn GameCardPassiveSkillHandler>);
                inner_map.insert(92, Box::new(NonePassiveSkillFunction) as Box<dyn GameCardPassiveSkillHandler>);
                inner_map.insert(112, Box::new(NonePassiveSkillFunction) as Box<dyn GameCardPassiveSkillHandler>);
                inner_map.insert(117, Box::new(NonePassiveSkillFunction) as Box<dyn GameCardPassiveSkillHandler>);
                inner_map.insert(120, Box::new(NonePassiveSkillFunction) as Box<dyn GameCardPassiveSkillHandler>);
                inner_map.insert(121, Box::new(NonePassiveSkillFunction) as Box<dyn GameCardPassiveSkillHandler>);
                inner_map.insert(127, Box::new(NonePassiveSkillFunction) as Box<dyn GameCardPassiveSkillHandler>);
                inner_map.insert(128, Box::new(NonePassiveSkillFunction) as Box<dyn GameCardPassiveSkillHandler>);
                inner_map.insert(130, Box::new(NonePassiveSkillFunction) as Box<dyn GameCardPassiveSkillHandler>);
                inner_map.insert(133, Box::new(NonePassiveSkillFunction) as Box<dyn GameCardPassiveSkillHandler>);
                inner_map.insert(134, Box::new(NonePassiveSkillFunction) as Box<dyn GameCardPassiveSkillHandler>);
                inner_map.insert(136, Box::new(NonePassiveSkillFunction) as Box<dyn GameCardPassiveSkillHandler>);
                inner_map.insert(139, Box::new(NonePassiveSkillFunction) as Box<dyn GameCardPassiveSkillHandler>);
                inner_map.insert(140, Box::new(NonePassiveSkillFunction) as Box<dyn GameCardPassiveSkillHandler>);
                inner_map.insert(160, Box::new(NonePassiveSkillFunction) as Box<dyn GameCardPassiveSkillHandler>);
                inner_map.insert(161, Box::new(NonePassiveSkillFunction) as Box<dyn GameCardPassiveSkillHandler>);
                inner_map.insert(162, Box::new(NonePassiveSkillFunction) as Box<dyn GameCardPassiveSkillHandler>);
                inner_map.insert(164, Box::new(NonePassiveSkillFunction) as Box<dyn GameCardPassiveSkillHandler>);
                inner_map.insert(171, Box::new(NonePassiveSkillFunction) as Box<dyn GameCardPassiveSkillHandler>);
                inner_map.insert(173, Box::new(NonePassiveSkillFunction) as Box<dyn GameCardPassiveSkillHandler>);
                inner_map.insert(174, Box::new(NonePassiveSkillFunction) as Box<dyn GameCardPassiveSkillHandler>);
                inner_map.insert(176, Box::new(NonePassiveSkillFunction) as Box<dyn GameCardPassiveSkillHandler>);
                inner_map.insert(177, Box::new(NonePassiveSkillFunction) as Box<dyn GameCardPassiveSkillHandler>);
                inner_map.insert(178, Box::new(NonePassiveSkillFunction) as Box<dyn GameCardPassiveSkillHandler>);
                inner_map.insert(179, Box::new(NonePassiveSkillFunction) as Box<dyn GameCardPassiveSkillHandler>);
                inner_map.insert(183, Box::new(NonePassiveSkillFunction) as Box<dyn GameCardPassiveSkillHandler>);
                inner_map.insert(184, Box::new(NonePassiveSkillFunction) as Box<dyn GameCardPassiveSkillHandler>);
                inner_map.insert(187, Box::new(NonePassiveSkillFunction) as Box<dyn GameCardPassiveSkillHandler>);
                inner_map.insert(189, Box::new(NonePassiveSkillFunction) as Box<dyn GameCardPassiveSkillHandler>);
                inner_map.insert(191, Box::new(NonePassiveSkillFunction) as Box<dyn GameCardPassiveSkillHandler>);
                inner_map.insert(193, Box::new(NonePassiveSkillFunction) as Box<dyn GameCardPassiveSkillHandler>);
                inner_map.insert(198, Box::new(NonePassiveSkillFunction) as Box<dyn GameCardPassiveSkillHandler>);
                inner_map.insert(200, Box::new(NonePassiveSkillFunction) as Box<dyn GameCardPassiveSkillHandler>);
                inner_map.insert(201, Box::new(NonePassiveSkillFunction) as Box<dyn GameCardPassiveSkillHandler>);
                inner_map.insert(202, Box::new(NonePassiveSkillFunction) as Box<dyn GameCardPassiveSkillHandler>);
            }
            3 => {
                inner_map.insert(6, Box::new(NonePassiveSkillFunction) as Box<dyn GameCardPassiveSkillHandler>);
                inner_map.insert(11, Box::new(NonePassiveSkillFunction) as Box<dyn GameCardPassiveSkillHandler>);
                inner_map.insert(17, Box::new(NonePassiveSkillFunction) as Box<dyn GameCardPassiveSkillHandler>);
                inner_map.insert(18, Box::new(NonePassiveSkillFunction) as Box<dyn GameCardPassiveSkillHandler>);
                inner_map.insert(19, Box::new(NonePassiveSkillFunction) as Box<dyn GameCardPassiveSkillHandler>);
                inner_map.insert(22, Box::new(NonePassiveSkillFunction) as Box<dyn GameCardPassiveSkillHandler>);
                inner_map.insert(23, Box::new(NonePassiveSkillFunction) as Box<dyn GameCardPassiveSkillHandler>);
                inner_map.insert(26, Box::new(NonePassiveSkillFunction) as Box<dyn GameCardPassiveSkillHandler>);
                inner_map.insert(27, Box::new(NonePassiveSkillFunction) as Box<dyn GameCardPassiveSkillHandler>);
                inner_map.insert(31, Box::new(NonePassiveSkillFunction) as Box<dyn GameCardPassiveSkillHandler>);
                inner_map.insert(32, Box::new(NonePassiveSkillFunction) as Box<dyn GameCardPassiveSkillHandler>);
                inner_map.insert(34, Box::new(NonePassiveSkillFunction) as Box<dyn GameCardPassiveSkillHandler>);
                inner_map.insert(38, Box::new(NonePassiveSkillFunction) as Box<dyn GameCardPassiveSkillHandler>);
                inner_map.insert(39, Box::new(NonePassiveSkillFunction) as Box<dyn GameCardPassiveSkillHandler>);
                inner_map.insert(40, Box::new(NonePassiveSkillFunction) as Box<dyn GameCardPassiveSkillHandler>);
                inner_map.insert(43, Box::new(NonePassiveSkillFunction) as Box<dyn GameCardPassiveSkillHandler>);
                inner_map.insert(44, Box::new(NonePassiveSkillFunction) as Box<dyn GameCardPassiveSkillHandler>);
                inner_map.insert(46, Box::new(NonePassiveSkillFunction) as Box<dyn GameCardPassiveSkillHandler>);
                inner_map.insert(48, Box::new(NonePassiveSkillFunction) as Box<dyn GameCardPassiveSkillHandler>);
                inner_map.insert(50, Box::new(NonePassiveSkillFunction) as Box<dyn GameCardPassiveSkillHandler>);
                inner_map.insert(51, Box::new(NonePassiveSkillFunction) as Box<dyn GameCardPassiveSkillHandler>);
                inner_map.insert(52, Box::new(NonePassiveSkillFunction) as Box<dyn GameCardPassiveSkillHandler>);
                inner_map.insert(53, Box::new(NonePassiveSkillFunction) as Box<dyn GameCardPassiveSkillHandler>);
                inner_map.insert(55, Box::new(NonePassiveSkillFunction) as Box<dyn GameCardPassiveSkillHandler>);
                inner_map.insert(56, Box::new(NonePassiveSkillFunction) as Box<dyn GameCardPassiveSkillHandler>);
                inner_map.insert(57, Box::new(NonePassiveSkillFunction) as Box<dyn GameCardPassiveSkillHandler>);
                inner_map.insert(58, Box::new(NonePassiveSkillFunction) as Box<dyn GameCardPassiveSkillHandler>);
                inner_map.insert(59, Box::new(NonePassiveSkillFunction) as Box<dyn GameCardPassiveSkillHandler>);
                inner_map.insert(60, Box::new(NonePassiveSkillFunction) as Box<dyn GameCardPassiveSkillHandler>);
                inner_map.insert(61, Box::new(NonePassiveSkillFunction) as Box<dyn GameCardPassiveSkillHandler>);
                inner_map.insert(63, Box::new(NonePassiveSkillFunction) as Box<dyn GameCardPassiveSkillHandler>);
                inner_map.insert(64, Box::new(NonePassiveSkillFunction) as Box<dyn GameCardPassiveSkillHandler>);
                inner_map.insert(70, Box::new(NonePassiveSkillFunction) as Box<dyn GameCardPassiveSkillHandler>);
                inner_map.insert(71, Box::new(NonePassiveSkillFunction) as Box<dyn GameCardPassiveSkillHandler>);
                inner_map.insert(72, Box::new(NonePassiveSkillFunction) as Box<dyn GameCardPassiveSkillHandler>);
                inner_map.insert(74, Box::new(NonePassiveSkillFunction) as Box<dyn GameCardPassiveSkillHandler>);
                inner_map.insert(75, Box::new(NonePassiveSkillFunction) as Box<dyn GameCardPassiveSkillHandler>);
                inner_map.insert(76, Box::new(NonePassiveSkillFunction) as Box<dyn GameCardPassiveSkillHandler>);
                inner_map.insert(78, Box::new(NonePassiveSkillFunction) as Box<dyn GameCardPassiveSkillHandler>);
                inner_map.insert(79, Box::new(NonePassiveSkillFunction) as Box<dyn GameCardPassiveSkillHandler>);
                inner_map.insert(80, Box::new(NonePassiveSkillFunction) as Box<dyn GameCardPassiveSkillHandler>);
                inner_map.insert(81, Box::new(NonePassiveSkillFunction) as Box<dyn GameCardPassiveSkillHandler>);
                inner_map.insert(82, Box::new(NonePassiveSkillFunction) as Box<dyn GameCardPassiveSkillHandler>);
                inner_map.insert(83, Box::new(NonePassiveSkillFunction) as Box<dyn GameCardPassiveSkillHandler>);
                inner_map.insert(84, Box::new(NonePassiveSkillFunction) as Box<dyn GameCardPassiveSkillHandler>);
                inner_map.insert(92, Box::new(NonePassiveSkillFunction) as Box<dyn GameCardPassiveSkillHandler>);
                inner_map.insert(112, Box::new(NonePassiveSkillFunction) as Box<dyn GameCardPassiveSkillHandler>);
                inner_map.insert(117, Box::new(NonePassiveSkillFunction) as Box<dyn GameCardPassiveSkillHandler>);
                inner_map.insert(120, Box::new(NonePassiveSkillFunction) as Box<dyn GameCardPassiveSkillHandler>);
                inner_map.insert(121, Box::new(NonePassiveSkillFunction) as Box<dyn GameCardPassiveSkillHandler>);
                inner_map.insert(127, Box::new(NonePassiveSkillFunction) as Box<dyn GameCardPassiveSkillHandler>);
                inner_map.insert(128, Box::new(NonePassiveSkillFunction) as Box<dyn GameCardPassiveSkillHandler>);
                inner_map.insert(130, Box::new(NonePassiveSkillFunction) as Box<dyn GameCardPassiveSkillHandler>);
                inner_map.insert(133, Box::new(NonePassiveSkillFunction) as Box<dyn GameCardPassiveSkillHandler>);
                inner_map.insert(134, Box::new(NonePassiveSkillFunction) as Box<dyn GameCardPassiveSkillHandler>);
                inner_map.insert(136, Box::new(NonePassiveSkillFunction) as Box<dyn GameCardPassiveSkillHandler>);
                inner_map.insert(139, Box::new(NonePassiveSkillFunction) as Box<dyn GameCardPassiveSkillHandler>);
                inner_map.insert(140, Box::new(NonePassiveSkillFunction) as Box<dyn GameCardPassiveSkillHandler>);
                inner_map.insert(160, Box::new(NonePassiveSkillFunction) as Box<dyn GameCardPassiveSkillHandler>);
                inner_map.insert(161, Box::new(NonePassiveSkillFunction) as Box<dyn GameCardPassiveSkillHandler>);
                inner_map.insert(162, Box::new(NonePassiveSkillFunction) as Box<dyn GameCardPassiveSkillHandler>);
                inner_map.insert(164, Box::new(NonePassiveSkillFunction) as Box<dyn GameCardPassiveSkillHandler>);
                inner_map.insert(171, Box::new(NonePassiveSkillFunction) as Box<dyn GameCardPassiveSkillHandler>);
                inner_map.insert(173, Box::new(NonePassiveSkillFunction) as Box<dyn GameCardPassiveSkillHandler>);
                inner_map.insert(174, Box::new(NonePassiveSkillFunction) as Box<dyn GameCardPassiveSkillHandler>);
                inner_map.insert(176, Box::new(NonePassiveSkillFunction) as Box<dyn GameCardPassiveSkillHandler>);
                inner_map.insert(177, Box::new(NonePassiveSkillFunction) as Box<dyn GameCardPassiveSkillHandler>);
                inner_map.insert(178, Box::new(NonePassiveSkillFunction) as Box<dyn GameCardPassiveSkillHandler>);
                inner_map.insert(179, Box::new(NonePassiveSkillFunction) as Box<dyn GameCardPassiveSkillHandler>);
                inner_map.insert(183, Box::new(NonePassiveSkillFunction) as Box<dyn GameCardPassiveSkillHandler>);
                inner_map.insert(184, Box::new(NonePassiveSkillFunction) as Box<dyn GameCardPassiveSkillHandler>);
                inner_map.insert(187, Box::new(NonePassiveSkillFunction) as Box<dyn GameCardPassiveSkillHandler>);
                inner_map.insert(189, Box::new(NonePassiveSkillFunction) as Box<dyn GameCardPassiveSkillHandler>);
                inner_map.insert(191, Box::new(NonePassiveSkillFunction) as Box<dyn GameCardPassiveSkillHandler>);
                inner_map.insert(193, Box::new(NonePassiveSkillFunction) as Box<dyn GameCardPassiveSkillHandler>);
                inner_map.insert(198, Box::new(NonePassiveSkillFunction) as Box<dyn GameCardPassiveSkillHandler>);
                inner_map.insert(200, Box::new(NonePassiveSkillFunction) as Box<dyn GameCardPassiveSkillHandler>);
                inner_map.insert(201, Box::new(NonePassiveSkillFunction) as Box<dyn GameCardPassiveSkillHandler>);
                inner_map.insert(202, Box::new(NonePassiveSkillFunction) as Box<dyn GameCardPassiveSkillHandler>);
            }
            _ => {}
        }

        inner_map
    }

    pub fn get_instance() -> Arc<AsyncMutex<GameCardPassiveSkillRepositoryImpl>> {
        lazy_static! {
            static ref INSTANCE: Arc<AsyncMutex<GameCardPassiveSkillRepositoryImpl>> =
                Arc::new(
                    AsyncMutex::new(
                        GameCardPassiveSkillRepositoryImpl::new()));
        }
        INSTANCE.clone()
    }
}

impl GameCardPassiveSkillRepository for GameCardPassiveSkillRepositoryImpl {
    unsafe fn call_passive_skill_repository_handler(&self, unit_card_id: i32, skill_index: i32) -> SummaryPassiveSkillEffect {
        println!("GameCardActiveSkillRepositoryImpl: call_active_skill_repository_handler()");

        if let Some(inner_map) = self.passive_skill_functions.get(&skill_index) {
            if let Some(passive_skill_summary_handler) = inner_map.get(&unit_card_id) {
                return passive_skill_summary_handler.summary_passive_skill();
            }
        }

        SummaryPassiveSkillEffect::new(PassiveSkillType::Dummy, vec![PassiveSkillCastingCondition::Dummy],-1)
    }
    unsafe fn call_deploy_passive_skill_repository_handler(&self, unit_card_id:i32, skill_index: i32) -> SummaryPassiveSkillEffect {
        println!("GameCardPassiveSkillRepositoryImpl: call_passive_skill_repository_handler()");

        if let Some(inner_map) = self.passive_skill_functions.get(&skill_index) {
            if let Some(passive_skill_summary_handler) = inner_map.get(&unit_card_id) {
                let is_deploy = passive_skill_summary_handler.summary_passive_skill()
                    .get_passive_skill_casting_condition()
                    .iter().find(|x| x == &&PassiveSkillCastingCondition::Deploy).is_some();
                if is_deploy { return passive_skill_summary_handler.summary_passive_skill(); }
            }
        }

        SummaryPassiveSkillEffect::new(PassiveSkillType::Dummy, vec![PassiveSkillCastingCondition::Dummy],-1)
    }
    unsafe fn call_turn_start_passive_skill_repository_handler(&self, unit_card_id:i32, skill_index: i32) -> SummaryPassiveSkillEffect {
        println!("GameCardPassiveSkillRepositoryImpl: call_passive_skill_repository_handler()");

        if let Some(inner_map) = self.passive_skill_functions.get(&skill_index) {
            if let Some(passive_skill_summary_handler) = inner_map.get(&unit_card_id) {
                let is_deploy = passive_skill_summary_handler.summary_passive_skill()
                    .get_passive_skill_casting_condition()
                    .iter().find(|x| x == &&PassiveSkillCastingCondition::TurnStart).is_some();
                if is_deploy { return passive_skill_summary_handler.summary_passive_skill(); }
            }
        }

        SummaryPassiveSkillEffect::new(PassiveSkillType::Dummy, vec![PassiveSkillCastingCondition::Dummy],-1)
    }
}