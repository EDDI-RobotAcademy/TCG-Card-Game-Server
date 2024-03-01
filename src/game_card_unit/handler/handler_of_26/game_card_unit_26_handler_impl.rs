use crate::common::card_attributes::card_grade::card_grade_enum::GradeEnum;
use crate::common::card_attributes::card_race::card_race_enum::RaceEnum;
use crate::game_card_unit::entity::game_card_unit_info::GameCardUnitInfo;
use crate::game_card_unit::entity::passive_status::PassiveStatus;
use crate::game_card_unit::handler::game_card_unit_handler::GameCardUnitHandler;

pub struct UnitCard_26_Function;

// race: RaceEnum,
// grade: GradeEnum,
// attack_point: i32,
// health_point: i32,
// attack_required_energy: i32,
// first_active_skill_required_energy: i32,
// second_active_skill_required_energy: i32,
// third_active_skill_required_energy: i32,
// first_passive_skill: i32,
// second_passive_skill: i32,
// third_passive_skill: i32,
// military_occupational_specialty: i32,

impl GameCardUnitHandler for UnitCard_26_Function {
    unsafe fn summary_unit_card(&self) -> GameCardUnitInfo {
        println!("UnitCard_26_Function: summary_unit_card()");

        let mut game_card_unit_info = GameCardUnitInfo::new(
            RaceEnum::Undead,
            GradeEnum::Uncommon,
            10,
            15,
            1,
            -1,
            -1,
            -1,
            true,
            true,
            false,
            -1);

        let mut passive_status_list_of_26 = Vec::new();
        passive_status_list_of_26.push(PassiveStatus::PhysicalImmunity);
        game_card_unit_info.set_passive_status_list(passive_status_list_of_26);

        return game_card_unit_info;
    }

    unsafe fn summary_unit_card_passive_default(&self) -> Vec<bool> {
        vec![true, true, false]
    }
}
