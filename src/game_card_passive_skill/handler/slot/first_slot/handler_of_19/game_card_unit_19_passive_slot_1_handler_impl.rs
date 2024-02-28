use crate::common::card_attributes::card_grade::card_grade_enum::GradeEnum;
use crate::common::card_attributes::card_race::card_race_enum::RaceEnum;
use crate::game_card_passive_skill::entity::passive_skill_type::PassiveSkillType;
use crate::game_card_passive_skill::entity::summary_passive_skill_effect::SummaryPassiveSkillEffect;
use crate::game_card_passive_skill::handler::game_card_passive_skill_handler::GameCardPassiveSkillHandler;
use crate::game_card_support::entity::game_card_support_effect::GameCardSupportEffect;
use crate::game_card_unit::entity::game_card_unit_info::GameCardUnitInfo;
use crate::game_card_unit::handler::game_card_unit_handler::GameCardUnitHandler;

pub struct UnitCard_19_Passive_Slot_1_Function;

// passive_skill_type: PassiveSkillType,
// skill_damage: i32,

impl GameCardPassiveSkillHandler for UnitCard_19_Passive_Slot_1_Function {
    unsafe fn summary_passive_skill(&self) -> SummaryPassiveSkillEffect {
        println!("UnitCard_19_Passive_Slot_1_Function: summary_passive_skill()");

        let mut summary_passive_skill_effect = SummaryPassiveSkillEffect::new(
            PassiveSkillType::BroadArea, 10);

        return summary_passive_skill_effect;
    }
}