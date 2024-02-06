use crate::game_first_active_skill::service::request::use_first_active_skill_request::UseFirstActiveSkillRequest;
use crate::game_first_active_skill::service::response::use_first_active_skill_response::UseFirstActiveSkillResponse;

pub trait GameFirstActiveSkillHandler {
    unsafe fn use_first_active_skill(&self, use_support_card_request: UseFirstActiveSkillRequest) -> UseFirstActiveSkillResponse;
}