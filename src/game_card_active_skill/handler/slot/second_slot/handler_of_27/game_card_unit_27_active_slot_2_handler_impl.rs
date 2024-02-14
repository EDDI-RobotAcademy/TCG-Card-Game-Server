use crate::game_card_active_skill::entity::active_skill_type::ActiveSkillType::BroadArea;
use crate::game_card_active_skill::entity::summary_active_skill_effect::SummaryActiveSkillEffect;
use crate::game_card_active_skill::handler::game_card_active_skill_handler::GameCardActiveSkillHandler;

pub struct UnitCard_27_Active_Slot_2_Function;

impl GameCardActiveSkillHandler for UnitCard_27_Active_Slot_2_Function {
    unsafe fn summary_active_skill(&self) -> SummaryActiveSkillEffect {
        let mut game_card_active_first_skill_effect = SummaryActiveSkillEffect::new(BroadArea, 15);

        game_card_active_first_skill_effect
    }
}