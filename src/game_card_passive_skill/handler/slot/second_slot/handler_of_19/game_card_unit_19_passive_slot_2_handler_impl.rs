use crate::game_card_passive_skill::entity::passive_skill_type::PassiveSkillType;
use crate::game_card_passive_skill::entity::summary_passive_skill_effect::SummaryPassiveSkillEffect;
use crate::game_card_passive_skill::handler::game_card_passive_skill_handler::GameCardPassiveSkillHandler;

pub struct UnitCard_19_Passive_Slot_2_Function;

// passive_skill_type: PassiveSkillType,
// skill_damage: i32,

impl GameCardPassiveSkillHandler for UnitCard_19_Passive_Slot_2_Function {
    unsafe fn summary_passive_skill(&self) -> SummaryPassiveSkillEffect {
        println!("UnitCard_19_Passive_Function: summary_passive_skill()");

        let mut summary_passive_skill_effect = SummaryPassiveSkillEffect::new(
            PassiveSkillType::SingleTarget, 20);

        return summary_passive_skill_effect;
    }
}