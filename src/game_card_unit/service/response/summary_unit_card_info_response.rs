use crate::common::card_attributes::card_grade::card_grade_enum::GradeEnum;
use crate::common::card_attributes::card_race::card_race_enum::RaceEnum;
use crate::game_card_unit::entity::game_card_unit_info::GameCardUnitInfo;
use crate::game_card_unit::entity::passive_status::PassiveStatus;

#[derive(Debug)]
pub struct SummaryUnitCardInfoResponse {
    unit_race: RaceEnum,
    unit_grade: GradeEnum,
    unit_attack_point: i32,
    unit_health_point: i32,
    unit_attack_required_energy: i32,
    first_active_skill_required_energy: i32,
    second_active_skill_required_energy: i32,
    third_active_skill_required_energy: i32,
    first_passive_skill: bool,
    second_passive_skill: bool,
    third_passive_skill: bool,
    // TODO: 병종은 나중에 생각하자
    passive_status_list: Vec<PassiveStatus>
}

impl SummaryUnitCardInfoResponse {
    pub fn new(
        unit_race: RaceEnum,
        unit_grade: GradeEnum,
        unit_attack_point: i32,
        unit_health_point: i32,
        unit_attack_required_energy: i32,
        first_active_skill_required_energy: i32,
        second_active_skill_required_energy: i32,
        third_active_skill_required_energy: i32,
        first_passive_skill: bool,
        second_passive_skill: bool,
        third_passive_skill: bool,
        passive_status_list: Vec<PassiveStatus> ) -> Self {

        Self {
            unit_race,
            unit_grade,
            unit_attack_point,
            unit_health_point,
            unit_attack_required_energy,
            first_active_skill_required_energy,
            second_active_skill_required_energy,
            third_active_skill_required_energy,
            first_passive_skill,
            second_passive_skill,
            third_passive_skill,
            passive_status_list }
    }

    pub fn get_unit_race(&self) -> RaceEnum {
        self.unit_race
    }

    pub fn get_unit_grade(&self) -> GradeEnum {
        self.unit_grade
    }

    pub fn get_unit_attack_point(&self) -> i32 {
        self.unit_attack_point
    }

    pub fn get_unit_health_point(&self) -> i32 {
        self.unit_health_point
    }

    pub fn get_unit_attack_required_energy(&self) -> i32 {
        self.unit_attack_required_energy
    }

    pub fn get_first_active_skill_required_energy(&self) -> i32 {
        self.first_active_skill_required_energy
    }

    pub fn get_second_active_skill_required_energy(&self) -> i32 {
        self.second_active_skill_required_energy
    }

    pub fn get_third_active_skill_required_energy(&self) -> i32 {
        self.third_active_skill_required_energy
    }

    pub fn has_first_passive_skill(&self) -> bool {
        self.first_passive_skill
    }

    pub fn has_second_passive_skill(&self) -> bool {
        self.second_passive_skill
    }

    pub fn has_third_passive_skill(&self) -> bool {
        self.third_passive_skill
    }

    pub fn get_passive_status_list(&self) -> &Vec<PassiveStatus> { &self.passive_status_list }

    pub fn from_game_card_unit_info(game_card_unit_info: GameCardUnitInfo) -> SummaryUnitCardInfoResponse {

        SummaryUnitCardInfoResponse::new(
            game_card_unit_info.get_race(),
            game_card_unit_info.get_grade(),
            game_card_unit_info.get_attack_point(),
            game_card_unit_info.get_health_point(),
            game_card_unit_info.get_attack_required_energy(),
            game_card_unit_info.get_first_active_skill_required_energy(),
            game_card_unit_info.get_second_active_skill_required_energy(),
            game_card_unit_info.get_third_active_skill_required_energy(),
            game_card_unit_info.has_first_passive_skill(),
            game_card_unit_info.has_second_passive_skill(),
            game_card_unit_info.has_third_passive_skill(),
            game_card_unit_info.get_passive_status_list().clone())
    }
}