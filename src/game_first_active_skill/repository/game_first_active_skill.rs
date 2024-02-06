use crate::game_first_active_skill::service::request::use_first_active_skill_request::UseFirstActiveSkillRequest;
use crate::game_first_active_skill::service::response::use_first_active_skill_response::UseFirstActiveSkillResponse;

pub trait GameFirstActiveSkillRepository {
    unsafe fn call_first_active_skill_repository_table(&self, use_first_active_skill_request: UseFirstActiveSkillRequest) -> UseFirstActiveSkillResponse;
}