use crate::common::card_attributes::card_race::card_race_enum::RaceEnum;
use crate::game_card_active_skill::entity::active_skill_type::ActiveSkillType;
use crate::game_card_active_skill::entity::summary_active_skill_effect::SummaryActiveSkillEffect;
use crate::game_card_active_skill::handler::game_card_active_skill_handler::GameCardActiveSkillHandler;

pub struct UnitCard_27_Active_Slot_1_Function;

impl GameCardActiveSkillHandler for UnitCard_27_Active_Slot_1_Function {
    unsafe fn summary_active_skill(&self) -> SummaryActiveSkillEffect {
        let mut game_card_active_first_skill_effect =
            SummaryActiveSkillEffect::new(RaceEnum::Undead,
                                          2,
                                          ActiveSkillType::SingleTarget,
                                          20);

        game_card_active_first_skill_effect
    }
}