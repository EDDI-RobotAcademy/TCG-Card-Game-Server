use crate::common::card_attributes::card_grade::card_grade_enum::GradeEnum;
use crate::common::card_attributes::card_race::card_race_enum::RaceEnum;
use crate::game_card_unit::entity::game_card_unit_info::GameCardUnitInfo;
use crate::game_card_unit::entity::passive_status::PassiveStatus;
use crate::game_card_unit::service::response::summary_unit_card_info_response::SummaryUnitCardInfoResponse;
use crate::game_field_unit::entity::extra_status_effect::ExtraStatusEffect;

#[derive(Debug)]
pub struct SummaryUnitCardPassiveDefaultResponse {
    passive_default_list: Vec<bool>
}

impl SummaryUnitCardPassiveDefaultResponse {
    pub fn new(passive_default_list: Vec<bool> ) -> Self {
        Self { passive_default_list }
    }
    pub fn get_passive_default_list(&self) -> &Vec<bool> {
        &self.passive_default_list
    }

}