use crate::game_card_active_skill::entity::active_skill_type::ActiveSkillType::SingleTarget;
use crate::game_card_active_skill::entity::summary_active_skill_effect::SummaryActiveSkillEffect;
use crate::game_card_active_skill::handler::game_card_active_skill_handler::GameCardActiveSkillHandler;

pub struct GameCardActiveFirstSkill_27_Function;

impl GameCardActiveSkillHandler for GameCardActiveFirstSkill_27_Function {
    unsafe fn summary_active_skill(&self) -> SummaryActiveSkillEffect {
        let mut game_card_active_first_skill_effect = SummaryActiveSkillEffect::new(SingleTarget, 20);

        game_card_active_first_skill_effect
    }
}