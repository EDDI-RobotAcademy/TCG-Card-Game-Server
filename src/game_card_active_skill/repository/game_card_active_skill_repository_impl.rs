use std::collections::HashMap;
use std::sync::Arc;
use lazy_static::lazy_static;

use tokio::sync::Mutex as AsyncMutex;
use crate::common::card_attributes::card_race::card_race_enum::RaceEnum;
use crate::game_card_active_skill::entity::active_skill_type::ActiveSkillType;
use crate::game_card_active_skill::entity::required_energy::RequiredEnergy;
use crate::game_card_active_skill::entity::summary_active_skill_effect::SummaryActiveSkillEffect;

use crate::game_card_active_skill::handler::game_card_active_skill_handler::GameCardActiveSkillHandler;
use crate::game_card_active_skill::repository::game_card_active_skill_repository::GameCardActiveSkillRepository;

use crate::game_card_active_skill::handler::slot::first_slot::handler_of_27::game_card_unit_27_active_slot_1_handler_impl::UnitCard_27_Active_Slot_1_Function;

use crate::game_card_active_skill::handler::slot::second_slot::handler_of_27::game_card_unit_27_active_slot_2_handler_impl::UnitCard_27_Active_Slot_2_Function;

pub struct GameCardActiveSkillRepositoryImpl {
    active_skill_functions: HashMap<i32, HashMap<i32, Box<dyn GameCardActiveSkillHandler>>>,
}

struct NoneActiveSkillFunction;

impl GameCardActiveSkillHandler for NoneActiveSkillFunction {
    unsafe fn summary_active_skill(&self) -> SummaryActiveSkillEffect {
        println!("아직 구현되지 않은 기능입니다.");

        SummaryActiveSkillEffect::new(
            RaceEnum::Dummy, -1, ActiveSkillType::Dummy, -1)
    }
}

impl GameCardActiveSkillRepositoryImpl {
    fn new() -> Self {
        let mut active_skill_functions = HashMap::new();

        for active_skill_index in &[1, 2, 3] {
            let inner_map = Self::create_inner_map_for_active_skill(*active_skill_index);
            active_skill_functions.insert(*active_skill_index, inner_map);
        }

        GameCardActiveSkillRepositoryImpl { active_skill_functions }
    }

    fn create_inner_map_for_active_skill(active_skill_index: i32) -> HashMap<i32, Box<dyn GameCardActiveSkillHandler>> {
        let mut inner_map = HashMap::new();

        match active_skill_index {
            1 => {
                inner_map.insert(6, Box::new(NoneActiveSkillFunction) as Box<dyn GameCardActiveSkillHandler>);
                inner_map.insert(11, Box::new(NoneActiveSkillFunction) as Box<dyn GameCardActiveSkillHandler>);
                inner_map.insert(17, Box::new(NoneActiveSkillFunction) as Box<dyn GameCardActiveSkillHandler>);
                inner_map.insert(18, Box::new(NoneActiveSkillFunction) as Box<dyn GameCardActiveSkillHandler>);
                inner_map.insert(19, Box::new(NoneActiveSkillFunction) as Box<dyn GameCardActiveSkillHandler>);
                inner_map.insert(22, Box::new(NoneActiveSkillFunction) as Box<dyn GameCardActiveSkillHandler>);
                inner_map.insert(23, Box::new(NoneActiveSkillFunction) as Box<dyn GameCardActiveSkillHandler>);
                inner_map.insert(26, Box::new(NoneActiveSkillFunction) as Box<dyn GameCardActiveSkillHandler>);
                inner_map.insert(27, Box::new(UnitCard_27_Active_Slot_1_Function) as Box<dyn GameCardActiveSkillHandler>);
                inner_map.insert(31, Box::new(NoneActiveSkillFunction) as Box<dyn GameCardActiveSkillHandler>);
                inner_map.insert(32, Box::new(NoneActiveSkillFunction) as Box<dyn GameCardActiveSkillHandler>);
                inner_map.insert(34, Box::new(NoneActiveSkillFunction) as Box<dyn GameCardActiveSkillHandler>);
                inner_map.insert(38, Box::new(NoneActiveSkillFunction) as Box<dyn GameCardActiveSkillHandler>);
                inner_map.insert(39, Box::new(NoneActiveSkillFunction) as Box<dyn GameCardActiveSkillHandler>);
                inner_map.insert(40, Box::new(NoneActiveSkillFunction) as Box<dyn GameCardActiveSkillHandler>);
                inner_map.insert(43, Box::new(NoneActiveSkillFunction) as Box<dyn GameCardActiveSkillHandler>);
                inner_map.insert(44, Box::new(NoneActiveSkillFunction) as Box<dyn GameCardActiveSkillHandler>);
                inner_map.insert(46, Box::new(NoneActiveSkillFunction) as Box<dyn GameCardActiveSkillHandler>);
                inner_map.insert(48, Box::new(NoneActiveSkillFunction) as Box<dyn GameCardActiveSkillHandler>);
                inner_map.insert(50, Box::new(NoneActiveSkillFunction) as Box<dyn GameCardActiveSkillHandler>);
                inner_map.insert(51, Box::new(NoneActiveSkillFunction) as Box<dyn GameCardActiveSkillHandler>);
                inner_map.insert(52, Box::new(NoneActiveSkillFunction) as Box<dyn GameCardActiveSkillHandler>);
                inner_map.insert(53, Box::new(NoneActiveSkillFunction) as Box<dyn GameCardActiveSkillHandler>);
                inner_map.insert(55, Box::new(NoneActiveSkillFunction) as Box<dyn GameCardActiveSkillHandler>);
                inner_map.insert(56, Box::new(NoneActiveSkillFunction) as Box<dyn GameCardActiveSkillHandler>);
                inner_map.insert(57, Box::new(NoneActiveSkillFunction) as Box<dyn GameCardActiveSkillHandler>);
                inner_map.insert(58, Box::new(NoneActiveSkillFunction) as Box<dyn GameCardActiveSkillHandler>);
                inner_map.insert(59, Box::new(NoneActiveSkillFunction) as Box<dyn GameCardActiveSkillHandler>);
                inner_map.insert(60, Box::new(NoneActiveSkillFunction) as Box<dyn GameCardActiveSkillHandler>);
                inner_map.insert(61, Box::new(NoneActiveSkillFunction) as Box<dyn GameCardActiveSkillHandler>);
                inner_map.insert(63, Box::new(NoneActiveSkillFunction) as Box<dyn GameCardActiveSkillHandler>);
                inner_map.insert(64, Box::new(NoneActiveSkillFunction) as Box<dyn GameCardActiveSkillHandler>);
                inner_map.insert(70, Box::new(NoneActiveSkillFunction) as Box<dyn GameCardActiveSkillHandler>);
                inner_map.insert(71, Box::new(NoneActiveSkillFunction) as Box<dyn GameCardActiveSkillHandler>);
                inner_map.insert(72, Box::new(NoneActiveSkillFunction) as Box<dyn GameCardActiveSkillHandler>);
                inner_map.insert(74, Box::new(NoneActiveSkillFunction) as Box<dyn GameCardActiveSkillHandler>);
                inner_map.insert(75, Box::new(NoneActiveSkillFunction) as Box<dyn GameCardActiveSkillHandler>);
                inner_map.insert(76, Box::new(NoneActiveSkillFunction) as Box<dyn GameCardActiveSkillHandler>);
                inner_map.insert(78, Box::new(NoneActiveSkillFunction) as Box<dyn GameCardActiveSkillHandler>);
                inner_map.insert(79, Box::new(NoneActiveSkillFunction) as Box<dyn GameCardActiveSkillHandler>);
                inner_map.insert(80, Box::new(NoneActiveSkillFunction) as Box<dyn GameCardActiveSkillHandler>);
                inner_map.insert(81, Box::new(NoneActiveSkillFunction) as Box<dyn GameCardActiveSkillHandler>);
                inner_map.insert(82, Box::new(NoneActiveSkillFunction) as Box<dyn GameCardActiveSkillHandler>);
                inner_map.insert(83, Box::new(NoneActiveSkillFunction) as Box<dyn GameCardActiveSkillHandler>);
                inner_map.insert(84, Box::new(NoneActiveSkillFunction) as Box<dyn GameCardActiveSkillHandler>);
                inner_map.insert(92, Box::new(NoneActiveSkillFunction) as Box<dyn GameCardActiveSkillHandler>);
                inner_map.insert(112, Box::new(NoneActiveSkillFunction) as Box<dyn GameCardActiveSkillHandler>);
                inner_map.insert(117, Box::new(NoneActiveSkillFunction) as Box<dyn GameCardActiveSkillHandler>);
                inner_map.insert(120, Box::new(NoneActiveSkillFunction) as Box<dyn GameCardActiveSkillHandler>);
                inner_map.insert(121, Box::new(NoneActiveSkillFunction) as Box<dyn GameCardActiveSkillHandler>);
                inner_map.insert(127, Box::new(NoneActiveSkillFunction) as Box<dyn GameCardActiveSkillHandler>);
                inner_map.insert(128, Box::new(NoneActiveSkillFunction) as Box<dyn GameCardActiveSkillHandler>);
                inner_map.insert(130, Box::new(NoneActiveSkillFunction) as Box<dyn GameCardActiveSkillHandler>);
                inner_map.insert(133, Box::new(NoneActiveSkillFunction) as Box<dyn GameCardActiveSkillHandler>);
                inner_map.insert(134, Box::new(NoneActiveSkillFunction) as Box<dyn GameCardActiveSkillHandler>);
                inner_map.insert(136, Box::new(NoneActiveSkillFunction) as Box<dyn GameCardActiveSkillHandler>);
                inner_map.insert(139, Box::new(NoneActiveSkillFunction) as Box<dyn GameCardActiveSkillHandler>);
                inner_map.insert(140, Box::new(NoneActiveSkillFunction) as Box<dyn GameCardActiveSkillHandler>);
                inner_map.insert(160, Box::new(NoneActiveSkillFunction) as Box<dyn GameCardActiveSkillHandler>);
                inner_map.insert(161, Box::new(NoneActiveSkillFunction) as Box<dyn GameCardActiveSkillHandler>);
                inner_map.insert(162, Box::new(NoneActiveSkillFunction) as Box<dyn GameCardActiveSkillHandler>);
                inner_map.insert(164, Box::new(NoneActiveSkillFunction) as Box<dyn GameCardActiveSkillHandler>);
                inner_map.insert(171, Box::new(NoneActiveSkillFunction) as Box<dyn GameCardActiveSkillHandler>);
                inner_map.insert(173, Box::new(NoneActiveSkillFunction) as Box<dyn GameCardActiveSkillHandler>);
                inner_map.insert(174, Box::new(NoneActiveSkillFunction) as Box<dyn GameCardActiveSkillHandler>);
                inner_map.insert(176, Box::new(NoneActiveSkillFunction) as Box<dyn GameCardActiveSkillHandler>);
                inner_map.insert(177, Box::new(NoneActiveSkillFunction) as Box<dyn GameCardActiveSkillHandler>);
                inner_map.insert(178, Box::new(NoneActiveSkillFunction) as Box<dyn GameCardActiveSkillHandler>);
                inner_map.insert(179, Box::new(NoneActiveSkillFunction) as Box<dyn GameCardActiveSkillHandler>);
                inner_map.insert(183, Box::new(NoneActiveSkillFunction) as Box<dyn GameCardActiveSkillHandler>);
                inner_map.insert(184, Box::new(NoneActiveSkillFunction) as Box<dyn GameCardActiveSkillHandler>);
                inner_map.insert(187, Box::new(NoneActiveSkillFunction) as Box<dyn GameCardActiveSkillHandler>);
                inner_map.insert(189, Box::new(NoneActiveSkillFunction) as Box<dyn GameCardActiveSkillHandler>);
                inner_map.insert(191, Box::new(NoneActiveSkillFunction) as Box<dyn GameCardActiveSkillHandler>);
                inner_map.insert(193, Box::new(NoneActiveSkillFunction) as Box<dyn GameCardActiveSkillHandler>);
                inner_map.insert(198, Box::new(NoneActiveSkillFunction) as Box<dyn GameCardActiveSkillHandler>);
                inner_map.insert(200, Box::new(NoneActiveSkillFunction) as Box<dyn GameCardActiveSkillHandler>);
                inner_map.insert(201, Box::new(NoneActiveSkillFunction) as Box<dyn GameCardActiveSkillHandler>);
                inner_map.insert(202, Box::new(NoneActiveSkillFunction) as Box<dyn GameCardActiveSkillHandler>);
            }
            2 => {
                inner_map.insert(6, Box::new(NoneActiveSkillFunction) as Box<dyn GameCardActiveSkillHandler>);
                inner_map.insert(11, Box::new(NoneActiveSkillFunction) as Box<dyn GameCardActiveSkillHandler>);
                inner_map.insert(17, Box::new(NoneActiveSkillFunction) as Box<dyn GameCardActiveSkillHandler>);
                inner_map.insert(18, Box::new(NoneActiveSkillFunction) as Box<dyn GameCardActiveSkillHandler>);
                inner_map.insert(19, Box::new(NoneActiveSkillFunction) as Box<dyn GameCardActiveSkillHandler>);
                inner_map.insert(22, Box::new(NoneActiveSkillFunction) as Box<dyn GameCardActiveSkillHandler>);
                inner_map.insert(23, Box::new(NoneActiveSkillFunction) as Box<dyn GameCardActiveSkillHandler>);
                inner_map.insert(26, Box::new(NoneActiveSkillFunction) as Box<dyn GameCardActiveSkillHandler>);
                inner_map.insert(27, Box::new(UnitCard_27_Active_Slot_2_Function) as Box<dyn GameCardActiveSkillHandler>);
                inner_map.insert(31, Box::new(NoneActiveSkillFunction) as Box<dyn GameCardActiveSkillHandler>);
                inner_map.insert(32, Box::new(NoneActiveSkillFunction) as Box<dyn GameCardActiveSkillHandler>);
                inner_map.insert(34, Box::new(NoneActiveSkillFunction) as Box<dyn GameCardActiveSkillHandler>);
                inner_map.insert(38, Box::new(NoneActiveSkillFunction) as Box<dyn GameCardActiveSkillHandler>);
                inner_map.insert(39, Box::new(NoneActiveSkillFunction) as Box<dyn GameCardActiveSkillHandler>);
                inner_map.insert(40, Box::new(NoneActiveSkillFunction) as Box<dyn GameCardActiveSkillHandler>);
                inner_map.insert(43, Box::new(NoneActiveSkillFunction) as Box<dyn GameCardActiveSkillHandler>);
                inner_map.insert(44, Box::new(NoneActiveSkillFunction) as Box<dyn GameCardActiveSkillHandler>);
                inner_map.insert(46, Box::new(NoneActiveSkillFunction) as Box<dyn GameCardActiveSkillHandler>);
                inner_map.insert(48, Box::new(NoneActiveSkillFunction) as Box<dyn GameCardActiveSkillHandler>);
                inner_map.insert(50, Box::new(NoneActiveSkillFunction) as Box<dyn GameCardActiveSkillHandler>);
                inner_map.insert(51, Box::new(NoneActiveSkillFunction) as Box<dyn GameCardActiveSkillHandler>);
                inner_map.insert(52, Box::new(NoneActiveSkillFunction) as Box<dyn GameCardActiveSkillHandler>);
                inner_map.insert(53, Box::new(NoneActiveSkillFunction) as Box<dyn GameCardActiveSkillHandler>);
                inner_map.insert(55, Box::new(NoneActiveSkillFunction) as Box<dyn GameCardActiveSkillHandler>);
                inner_map.insert(56, Box::new(NoneActiveSkillFunction) as Box<dyn GameCardActiveSkillHandler>);
                inner_map.insert(57, Box::new(NoneActiveSkillFunction) as Box<dyn GameCardActiveSkillHandler>);
                inner_map.insert(58, Box::new(NoneActiveSkillFunction) as Box<dyn GameCardActiveSkillHandler>);
                inner_map.insert(59, Box::new(NoneActiveSkillFunction) as Box<dyn GameCardActiveSkillHandler>);
                inner_map.insert(60, Box::new(NoneActiveSkillFunction) as Box<dyn GameCardActiveSkillHandler>);
                inner_map.insert(61, Box::new(NoneActiveSkillFunction) as Box<dyn GameCardActiveSkillHandler>);
                inner_map.insert(63, Box::new(NoneActiveSkillFunction) as Box<dyn GameCardActiveSkillHandler>);
                inner_map.insert(64, Box::new(NoneActiveSkillFunction) as Box<dyn GameCardActiveSkillHandler>);
                inner_map.insert(70, Box::new(NoneActiveSkillFunction) as Box<dyn GameCardActiveSkillHandler>);
                inner_map.insert(71, Box::new(NoneActiveSkillFunction) as Box<dyn GameCardActiveSkillHandler>);
                inner_map.insert(72, Box::new(NoneActiveSkillFunction) as Box<dyn GameCardActiveSkillHandler>);
                inner_map.insert(74, Box::new(NoneActiveSkillFunction) as Box<dyn GameCardActiveSkillHandler>);
                inner_map.insert(75, Box::new(NoneActiveSkillFunction) as Box<dyn GameCardActiveSkillHandler>);
                inner_map.insert(76, Box::new(NoneActiveSkillFunction) as Box<dyn GameCardActiveSkillHandler>);
                inner_map.insert(78, Box::new(NoneActiveSkillFunction) as Box<dyn GameCardActiveSkillHandler>);
                inner_map.insert(79, Box::new(NoneActiveSkillFunction) as Box<dyn GameCardActiveSkillHandler>);
                inner_map.insert(80, Box::new(NoneActiveSkillFunction) as Box<dyn GameCardActiveSkillHandler>);
                inner_map.insert(81, Box::new(NoneActiveSkillFunction) as Box<dyn GameCardActiveSkillHandler>);
                inner_map.insert(82, Box::new(NoneActiveSkillFunction) as Box<dyn GameCardActiveSkillHandler>);
                inner_map.insert(83, Box::new(NoneActiveSkillFunction) as Box<dyn GameCardActiveSkillHandler>);
                inner_map.insert(84, Box::new(NoneActiveSkillFunction) as Box<dyn GameCardActiveSkillHandler>);
                inner_map.insert(92, Box::new(NoneActiveSkillFunction) as Box<dyn GameCardActiveSkillHandler>);
                inner_map.insert(112, Box::new(NoneActiveSkillFunction) as Box<dyn GameCardActiveSkillHandler>);
                inner_map.insert(117, Box::new(NoneActiveSkillFunction) as Box<dyn GameCardActiveSkillHandler>);
                inner_map.insert(120, Box::new(NoneActiveSkillFunction) as Box<dyn GameCardActiveSkillHandler>);
                inner_map.insert(121, Box::new(NoneActiveSkillFunction) as Box<dyn GameCardActiveSkillHandler>);
                inner_map.insert(127, Box::new(NoneActiveSkillFunction) as Box<dyn GameCardActiveSkillHandler>);
                inner_map.insert(128, Box::new(NoneActiveSkillFunction) as Box<dyn GameCardActiveSkillHandler>);
                inner_map.insert(130, Box::new(NoneActiveSkillFunction) as Box<dyn GameCardActiveSkillHandler>);
                inner_map.insert(133, Box::new(NoneActiveSkillFunction) as Box<dyn GameCardActiveSkillHandler>);
                inner_map.insert(134, Box::new(NoneActiveSkillFunction) as Box<dyn GameCardActiveSkillHandler>);
                inner_map.insert(136, Box::new(NoneActiveSkillFunction) as Box<dyn GameCardActiveSkillHandler>);
                inner_map.insert(139, Box::new(NoneActiveSkillFunction) as Box<dyn GameCardActiveSkillHandler>);
                inner_map.insert(140, Box::new(NoneActiveSkillFunction) as Box<dyn GameCardActiveSkillHandler>);
                inner_map.insert(160, Box::new(NoneActiveSkillFunction) as Box<dyn GameCardActiveSkillHandler>);
                inner_map.insert(161, Box::new(NoneActiveSkillFunction) as Box<dyn GameCardActiveSkillHandler>);
                inner_map.insert(162, Box::new(NoneActiveSkillFunction) as Box<dyn GameCardActiveSkillHandler>);
                inner_map.insert(164, Box::new(NoneActiveSkillFunction) as Box<dyn GameCardActiveSkillHandler>);
                inner_map.insert(171, Box::new(NoneActiveSkillFunction) as Box<dyn GameCardActiveSkillHandler>);
                inner_map.insert(173, Box::new(NoneActiveSkillFunction) as Box<dyn GameCardActiveSkillHandler>);
                inner_map.insert(174, Box::new(NoneActiveSkillFunction) as Box<dyn GameCardActiveSkillHandler>);
                inner_map.insert(176, Box::new(NoneActiveSkillFunction) as Box<dyn GameCardActiveSkillHandler>);
                inner_map.insert(177, Box::new(NoneActiveSkillFunction) as Box<dyn GameCardActiveSkillHandler>);
                inner_map.insert(178, Box::new(NoneActiveSkillFunction) as Box<dyn GameCardActiveSkillHandler>);
                inner_map.insert(179, Box::new(NoneActiveSkillFunction) as Box<dyn GameCardActiveSkillHandler>);
                inner_map.insert(183, Box::new(NoneActiveSkillFunction) as Box<dyn GameCardActiveSkillHandler>);
                inner_map.insert(184, Box::new(NoneActiveSkillFunction) as Box<dyn GameCardActiveSkillHandler>);
                inner_map.insert(187, Box::new(NoneActiveSkillFunction) as Box<dyn GameCardActiveSkillHandler>);
                inner_map.insert(189, Box::new(NoneActiveSkillFunction) as Box<dyn GameCardActiveSkillHandler>);
                inner_map.insert(191, Box::new(NoneActiveSkillFunction) as Box<dyn GameCardActiveSkillHandler>);
                inner_map.insert(193, Box::new(NoneActiveSkillFunction) as Box<dyn GameCardActiveSkillHandler>);
                inner_map.insert(198, Box::new(NoneActiveSkillFunction) as Box<dyn GameCardActiveSkillHandler>);
                inner_map.insert(200, Box::new(NoneActiveSkillFunction) as Box<dyn GameCardActiveSkillHandler>);
                inner_map.insert(201, Box::new(NoneActiveSkillFunction) as Box<dyn GameCardActiveSkillHandler>);
                inner_map.insert(202, Box::new(NoneActiveSkillFunction) as Box<dyn GameCardActiveSkillHandler>);
            }
            3 => {
                inner_map.insert(6, Box::new(NoneActiveSkillFunction) as Box<dyn GameCardActiveSkillHandler>);
                inner_map.insert(11, Box::new(NoneActiveSkillFunction) as Box<dyn GameCardActiveSkillHandler>);
                inner_map.insert(17, Box::new(NoneActiveSkillFunction) as Box<dyn GameCardActiveSkillHandler>);
                inner_map.insert(18, Box::new(NoneActiveSkillFunction) as Box<dyn GameCardActiveSkillHandler>);
                inner_map.insert(19, Box::new(NoneActiveSkillFunction) as Box<dyn GameCardActiveSkillHandler>);
                inner_map.insert(22, Box::new(NoneActiveSkillFunction) as Box<dyn GameCardActiveSkillHandler>);
                inner_map.insert(23, Box::new(NoneActiveSkillFunction) as Box<dyn GameCardActiveSkillHandler>);
                inner_map.insert(26, Box::new(NoneActiveSkillFunction) as Box<dyn GameCardActiveSkillHandler>);
                inner_map.insert(27, Box::new(NoneActiveSkillFunction) as Box<dyn GameCardActiveSkillHandler>);
                inner_map.insert(31, Box::new(NoneActiveSkillFunction) as Box<dyn GameCardActiveSkillHandler>);
                inner_map.insert(32, Box::new(NoneActiveSkillFunction) as Box<dyn GameCardActiveSkillHandler>);
                inner_map.insert(34, Box::new(NoneActiveSkillFunction) as Box<dyn GameCardActiveSkillHandler>);
                inner_map.insert(38, Box::new(NoneActiveSkillFunction) as Box<dyn GameCardActiveSkillHandler>);
                inner_map.insert(39, Box::new(NoneActiveSkillFunction) as Box<dyn GameCardActiveSkillHandler>);
                inner_map.insert(40, Box::new(NoneActiveSkillFunction) as Box<dyn GameCardActiveSkillHandler>);
                inner_map.insert(43, Box::new(NoneActiveSkillFunction) as Box<dyn GameCardActiveSkillHandler>);
                inner_map.insert(44, Box::new(NoneActiveSkillFunction) as Box<dyn GameCardActiveSkillHandler>);
                inner_map.insert(46, Box::new(NoneActiveSkillFunction) as Box<dyn GameCardActiveSkillHandler>);
                inner_map.insert(48, Box::new(NoneActiveSkillFunction) as Box<dyn GameCardActiveSkillHandler>);
                inner_map.insert(50, Box::new(NoneActiveSkillFunction) as Box<dyn GameCardActiveSkillHandler>);
                inner_map.insert(51, Box::new(NoneActiveSkillFunction) as Box<dyn GameCardActiveSkillHandler>);
                inner_map.insert(52, Box::new(NoneActiveSkillFunction) as Box<dyn GameCardActiveSkillHandler>);
                inner_map.insert(53, Box::new(NoneActiveSkillFunction) as Box<dyn GameCardActiveSkillHandler>);
                inner_map.insert(55, Box::new(NoneActiveSkillFunction) as Box<dyn GameCardActiveSkillHandler>);
                inner_map.insert(56, Box::new(NoneActiveSkillFunction) as Box<dyn GameCardActiveSkillHandler>);
                inner_map.insert(57, Box::new(NoneActiveSkillFunction) as Box<dyn GameCardActiveSkillHandler>);
                inner_map.insert(58, Box::new(NoneActiveSkillFunction) as Box<dyn GameCardActiveSkillHandler>);
                inner_map.insert(59, Box::new(NoneActiveSkillFunction) as Box<dyn GameCardActiveSkillHandler>);
                inner_map.insert(60, Box::new(NoneActiveSkillFunction) as Box<dyn GameCardActiveSkillHandler>);
                inner_map.insert(61, Box::new(NoneActiveSkillFunction) as Box<dyn GameCardActiveSkillHandler>);
                inner_map.insert(63, Box::new(NoneActiveSkillFunction) as Box<dyn GameCardActiveSkillHandler>);
                inner_map.insert(64, Box::new(NoneActiveSkillFunction) as Box<dyn GameCardActiveSkillHandler>);
                inner_map.insert(70, Box::new(NoneActiveSkillFunction) as Box<dyn GameCardActiveSkillHandler>);
                inner_map.insert(71, Box::new(NoneActiveSkillFunction) as Box<dyn GameCardActiveSkillHandler>);
                inner_map.insert(72, Box::new(NoneActiveSkillFunction) as Box<dyn GameCardActiveSkillHandler>);
                inner_map.insert(74, Box::new(NoneActiveSkillFunction) as Box<dyn GameCardActiveSkillHandler>);
                inner_map.insert(75, Box::new(NoneActiveSkillFunction) as Box<dyn GameCardActiveSkillHandler>);
                inner_map.insert(76, Box::new(NoneActiveSkillFunction) as Box<dyn GameCardActiveSkillHandler>);
                inner_map.insert(78, Box::new(NoneActiveSkillFunction) as Box<dyn GameCardActiveSkillHandler>);
                inner_map.insert(79, Box::new(NoneActiveSkillFunction) as Box<dyn GameCardActiveSkillHandler>);
                inner_map.insert(80, Box::new(NoneActiveSkillFunction) as Box<dyn GameCardActiveSkillHandler>);
                inner_map.insert(81, Box::new(NoneActiveSkillFunction) as Box<dyn GameCardActiveSkillHandler>);
                inner_map.insert(82, Box::new(NoneActiveSkillFunction) as Box<dyn GameCardActiveSkillHandler>);
                inner_map.insert(83, Box::new(NoneActiveSkillFunction) as Box<dyn GameCardActiveSkillHandler>);
                inner_map.insert(84, Box::new(NoneActiveSkillFunction) as Box<dyn GameCardActiveSkillHandler>);
                inner_map.insert(92, Box::new(NoneActiveSkillFunction) as Box<dyn GameCardActiveSkillHandler>);
                inner_map.insert(112, Box::new(NoneActiveSkillFunction) as Box<dyn GameCardActiveSkillHandler>);
                inner_map.insert(117, Box::new(NoneActiveSkillFunction) as Box<dyn GameCardActiveSkillHandler>);
                inner_map.insert(120, Box::new(NoneActiveSkillFunction) as Box<dyn GameCardActiveSkillHandler>);
                inner_map.insert(121, Box::new(NoneActiveSkillFunction) as Box<dyn GameCardActiveSkillHandler>);
                inner_map.insert(127, Box::new(NoneActiveSkillFunction) as Box<dyn GameCardActiveSkillHandler>);
                inner_map.insert(128, Box::new(NoneActiveSkillFunction) as Box<dyn GameCardActiveSkillHandler>);
                inner_map.insert(130, Box::new(NoneActiveSkillFunction) as Box<dyn GameCardActiveSkillHandler>);
                inner_map.insert(133, Box::new(NoneActiveSkillFunction) as Box<dyn GameCardActiveSkillHandler>);
                inner_map.insert(134, Box::new(NoneActiveSkillFunction) as Box<dyn GameCardActiveSkillHandler>);
                inner_map.insert(136, Box::new(NoneActiveSkillFunction) as Box<dyn GameCardActiveSkillHandler>);
                inner_map.insert(139, Box::new(NoneActiveSkillFunction) as Box<dyn GameCardActiveSkillHandler>);
                inner_map.insert(140, Box::new(NoneActiveSkillFunction) as Box<dyn GameCardActiveSkillHandler>);
                inner_map.insert(160, Box::new(NoneActiveSkillFunction) as Box<dyn GameCardActiveSkillHandler>);
                inner_map.insert(161, Box::new(NoneActiveSkillFunction) as Box<dyn GameCardActiveSkillHandler>);
                inner_map.insert(162, Box::new(NoneActiveSkillFunction) as Box<dyn GameCardActiveSkillHandler>);
                inner_map.insert(164, Box::new(NoneActiveSkillFunction) as Box<dyn GameCardActiveSkillHandler>);
                inner_map.insert(171, Box::new(NoneActiveSkillFunction) as Box<dyn GameCardActiveSkillHandler>);
                inner_map.insert(173, Box::new(NoneActiveSkillFunction) as Box<dyn GameCardActiveSkillHandler>);
                inner_map.insert(174, Box::new(NoneActiveSkillFunction) as Box<dyn GameCardActiveSkillHandler>);
                inner_map.insert(176, Box::new(NoneActiveSkillFunction) as Box<dyn GameCardActiveSkillHandler>);
                inner_map.insert(177, Box::new(NoneActiveSkillFunction) as Box<dyn GameCardActiveSkillHandler>);
                inner_map.insert(178, Box::new(NoneActiveSkillFunction) as Box<dyn GameCardActiveSkillHandler>);
                inner_map.insert(179, Box::new(NoneActiveSkillFunction) as Box<dyn GameCardActiveSkillHandler>);
                inner_map.insert(183, Box::new(NoneActiveSkillFunction) as Box<dyn GameCardActiveSkillHandler>);
                inner_map.insert(184, Box::new(NoneActiveSkillFunction) as Box<dyn GameCardActiveSkillHandler>);
                inner_map.insert(187, Box::new(NoneActiveSkillFunction) as Box<dyn GameCardActiveSkillHandler>);
                inner_map.insert(189, Box::new(NoneActiveSkillFunction) as Box<dyn GameCardActiveSkillHandler>);
                inner_map.insert(191, Box::new(NoneActiveSkillFunction) as Box<dyn GameCardActiveSkillHandler>);
                inner_map.insert(193, Box::new(NoneActiveSkillFunction) as Box<dyn GameCardActiveSkillHandler>);
                inner_map.insert(198, Box::new(NoneActiveSkillFunction) as Box<dyn GameCardActiveSkillHandler>);
                inner_map.insert(200, Box::new(NoneActiveSkillFunction) as Box<dyn GameCardActiveSkillHandler>);
                inner_map.insert(201, Box::new(NoneActiveSkillFunction) as Box<dyn GameCardActiveSkillHandler>);
                inner_map.insert(202, Box::new(NoneActiveSkillFunction) as Box<dyn GameCardActiveSkillHandler>);
            }
            _ => {}
        }

        inner_map
    }

    pub fn get_instance() -> Arc<AsyncMutex<GameCardActiveSkillRepositoryImpl>> {
        lazy_static! {
            static ref INSTANCE: Arc<AsyncMutex<GameCardActiveSkillRepositoryImpl>> =
                Arc::new(
                    AsyncMutex::new(
                        GameCardActiveSkillRepositoryImpl::new()));
        }
        INSTANCE.clone()
    }
}

impl GameCardActiveSkillRepository for GameCardActiveSkillRepositoryImpl {
    unsafe fn call_active_skill_repository_handler(&self, unit_card_id: i32, skill_index: i32) -> SummaryActiveSkillEffect {
        println!("GameCardActiveSkillRepositoryImpl: call_active_skill_repository_handler()");

        if let Some(inner_map) = self.active_skill_functions.get(&skill_index) {
            if let Some(active_skill_summary_handler) = inner_map.get(&unit_card_id) {
                return active_skill_summary_handler.summary_active_skill();
            }
        }

        SummaryActiveSkillEffect::new(
            RaceEnum::Dummy, -1, ActiveSkillType::Dummy, -1)
    }
}