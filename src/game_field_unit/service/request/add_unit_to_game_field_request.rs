use crate::common::card_attributes::card_grade::card_grade_enum::GradeEnum;
use crate::common::card_attributes::card_race::card_race_enum::RaceEnum;

#[derive(Debug)]
pub struct AddUnitToGameFieldRequest {
    account_unique_id: i32,
    unit_card_id: i32,
    unit_race: RaceEnum,
    unit_grade: GradeEnum,
    unit_attack_point: i32,
    unit_health_point: i32,
    unit_attack_required_energy: i32,
    has_first_passive_skill: bool,
    has_second_passive_skill: bool,
    has_third_passive_skill: bool
}

impl AddUnitToGameFieldRequest {
    pub fn new(account_unique_id: i32,
               unit_card_id: i32,
               unit_race: RaceEnum,
               unit_grade: GradeEnum,
               unit_attack_point: i32,
               unit_health_point: i32,
               unit_attack_required_energy: i32,
               has_first_passive_skill: bool,
               has_second_passive_skill: bool,
               has_third_passive_skill: bool) -> Self {

        AddUnitToGameFieldRequest {
            account_unique_id,
            unit_card_id,
            unit_race,
            unit_grade,
            unit_attack_point,
            unit_health_point,
            unit_attack_required_energy,
            has_first_passive_skill,
            has_second_passive_skill,
            has_third_passive_skill
        }
    }

    pub fn get_account_unique_id(&self) -> i32 {
        self.account_unique_id
    }

    pub fn get_unit_card_id(&self) -> i32 {
        self.unit_card_id
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

    pub fn has_first_passive_skill(&self) -> bool {
        self.has_first_passive_skill
    }

    pub fn has_second_passive_skill(&self) -> bool {
        self.has_second_passive_skill
    }

    pub fn has_third_passive_skill(&self) -> bool {
        self.has_third_passive_skill
    }
}
