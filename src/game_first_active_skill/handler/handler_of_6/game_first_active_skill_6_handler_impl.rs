use crate::game_first_active_skill::handler::game_first_active_skill_handler::GameFirstActiveSkillHandler;
use crate::game_first_active_skill::service::request::use_first_active_skill_request::UseFirstActiveSkillRequest;
use crate::game_first_active_skill::service::response::use_first_active_skill_response::UseFirstActiveSkillResponse;

pub struct FirstActiveSkill_6_Function;

impl GameFirstActiveSkillHandler for FirstActiveSkill_6_Function {
    unsafe fn use_first_active_skill(&self, use_support_card_request: UseFirstActiveSkillRequest) -> UseFirstActiveSkillResponse {
        println!("FirstActiveSkill_6_Function: use_first_active_skill()");

        UseFirstActiveSkillResponse
    }
}
