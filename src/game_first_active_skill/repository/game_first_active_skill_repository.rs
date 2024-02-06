use std::collections::HashMap;
use crate::game_first_active_skill::handler::game_first_active_skill_handler::GameFirstActiveSkillHandler;
use crate::game_first_active_skill::handler::handler_of_6::game_first_active_skill_6_handler_impl::FirstActiveSkill_6_Function;
use crate::game_first_active_skill::repository::game_first_active_skill::GameFirstActiveSkillRepository;
use crate::game_first_active_skill::service::request::use_first_active_skill_request::UseFirstActiveSkillRequest;
use crate::game_first_active_skill::service::response::use_first_active_skill_response::UseFirstActiveSkillResponse;

pub struct GameFirstActiveSkillRepositoryImpl {
    first_active_skill_functions: HashMap<i32, Box<dyn GameFirstActiveSkillHandler>>,
}

struct NoneFunction;

impl GameFirstActiveSkillHandler for NoneFunction {

    unsafe fn use_first_active_skill(&self, use_support_card_request: UseFirstActiveSkillRequest) -> UseFirstActiveSkillResponse {
        println!("아직 구현되지 않은 기능입니다.");

        UseFirstActiveSkillResponse
    }
}

impl GameFirstActiveSkillRepositoryImpl {
    fn new() -> Self {
        let mut first_active_skill_functions = HashMap::new();
        first_active_skill_functions.insert(6, Box::new(FirstActiveSkill_6_Function) as Box<dyn GameFirstActiveSkillHandler>);
        first_active_skill_functions.insert(11, Box::new(NoneFunction) as Box<dyn GameFirstActiveSkillHandler>);
        first_active_skill_functions.insert(17, Box::new(NoneFunction) as Box<dyn GameFirstActiveSkillHandler>);
        first_active_skill_functions.insert(18, Box::new(NoneFunction) as Box<dyn GameFirstActiveSkillHandler>);
        first_active_skill_functions.insert(19, Box::new(NoneFunction) as Box<dyn GameFirstActiveSkillHandler>);
        first_active_skill_functions.insert(22, Box::new(NoneFunction) as Box<dyn GameFirstActiveSkillHandler>);
        first_active_skill_functions.insert(23, Box::new(NoneFunction) as Box<dyn GameFirstActiveSkillHandler>);
        first_active_skill_functions.insert(26, Box::new(NoneFunction) as Box<dyn GameFirstActiveSkillHandler>);
        first_active_skill_functions.insert(27, Box::new(NoneFunction) as Box<dyn GameFirstActiveSkillHandler>);
        first_active_skill_functions.insert(31, Box::new(NoneFunction) as Box<dyn GameFirstActiveSkillHandler>);
        first_active_skill_functions.insert(32, Box::new(NoneFunction) as Box<dyn GameFirstActiveSkillHandler>);
        first_active_skill_functions.insert(34, Box::new(NoneFunction) as Box<dyn GameFirstActiveSkillHandler>);
        first_active_skill_functions.insert(38, Box::new(NoneFunction) as Box<dyn GameFirstActiveSkillHandler>);
        first_active_skill_functions.insert(39, Box::new(NoneFunction) as Box<dyn GameFirstActiveSkillHandler>);
        first_active_skill_functions.insert(40, Box::new(NoneFunction) as Box<dyn GameFirstActiveSkillHandler>);
        first_active_skill_functions.insert(43, Box::new(NoneFunction) as Box<dyn GameFirstActiveSkillHandler>);
        first_active_skill_functions.insert(44, Box::new(NoneFunction) as Box<dyn GameFirstActiveSkillHandler>);
        first_active_skill_functions.insert(46, Box::new(NoneFunction) as Box<dyn GameFirstActiveSkillHandler>);
        first_active_skill_functions.insert(48, Box::new(NoneFunction) as Box<dyn GameFirstActiveSkillHandler>);
        first_active_skill_functions.insert(50, Box::new(NoneFunction) as Box<dyn GameFirstActiveSkillHandler>);
        first_active_skill_functions.insert(51, Box::new(NoneFunction) as Box<dyn GameFirstActiveSkillHandler>);
        first_active_skill_functions.insert(52, Box::new(NoneFunction) as Box<dyn GameFirstActiveSkillHandler>);
        first_active_skill_functions.insert(53, Box::new(NoneFunction) as Box<dyn GameFirstActiveSkillHandler>);
        first_active_skill_functions.insert(55, Box::new(NoneFunction) as Box<dyn GameFirstActiveSkillHandler>);
        first_active_skill_functions.insert(56, Box::new(NoneFunction) as Box<dyn GameFirstActiveSkillHandler>);
        first_active_skill_functions.insert(57, Box::new(NoneFunction) as Box<dyn GameFirstActiveSkillHandler>);
        first_active_skill_functions.insert(58, Box::new(NoneFunction) as Box<dyn GameFirstActiveSkillHandler>);
        first_active_skill_functions.insert(59, Box::new(NoneFunction) as Box<dyn GameFirstActiveSkillHandler>);
        first_active_skill_functions.insert(60, Box::new(NoneFunction) as Box<dyn GameFirstActiveSkillHandler>);
        first_active_skill_functions.insert(61, Box::new(NoneFunction) as Box<dyn GameFirstActiveSkillHandler>);
        first_active_skill_functions.insert(63, Box::new(NoneFunction) as Box<dyn GameFirstActiveSkillHandler>);
        first_active_skill_functions.insert(64, Box::new(NoneFunction) as Box<dyn GameFirstActiveSkillHandler>);
        first_active_skill_functions.insert(70, Box::new(NoneFunction) as Box<dyn GameFirstActiveSkillHandler>);
        first_active_skill_functions.insert(71, Box::new(NoneFunction) as Box<dyn GameFirstActiveSkillHandler>);
        first_active_skill_functions.insert(72, Box::new(NoneFunction) as Box<dyn GameFirstActiveSkillHandler>);
        first_active_skill_functions.insert(74, Box::new(NoneFunction) as Box<dyn GameFirstActiveSkillHandler>);
        first_active_skill_functions.insert(75, Box::new(NoneFunction) as Box<dyn GameFirstActiveSkillHandler>);
        first_active_skill_functions.insert(76, Box::new(NoneFunction) as Box<dyn GameFirstActiveSkillHandler>);
        first_active_skill_functions.insert(78, Box::new(NoneFunction) as Box<dyn GameFirstActiveSkillHandler>);
        first_active_skill_functions.insert(79, Box::new(NoneFunction) as Box<dyn GameFirstActiveSkillHandler>);
        first_active_skill_functions.insert(80, Box::new(NoneFunction) as Box<dyn GameFirstActiveSkillHandler>);
        first_active_skill_functions.insert(81, Box::new(NoneFunction) as Box<dyn GameFirstActiveSkillHandler>);
        first_active_skill_functions.insert(82, Box::new(NoneFunction) as Box<dyn GameFirstActiveSkillHandler>);
        first_active_skill_functions.insert(83, Box::new(NoneFunction) as Box<dyn GameFirstActiveSkillHandler>);
        first_active_skill_functions.insert(84, Box::new(NoneFunction) as Box<dyn GameFirstActiveSkillHandler>);
        first_active_skill_functions.insert(92, Box::new(NoneFunction) as Box<dyn GameFirstActiveSkillHandler>);
        first_active_skill_functions.insert(112, Box::new(NoneFunction) as Box<dyn GameFirstActiveSkillHandler>);
        first_active_skill_functions.insert(117, Box::new(NoneFunction) as Box<dyn GameFirstActiveSkillHandler>);
        first_active_skill_functions.insert(120, Box::new(NoneFunction) as Box<dyn GameFirstActiveSkillHandler>);
        first_active_skill_functions.insert(121, Box::new(NoneFunction) as Box<dyn GameFirstActiveSkillHandler>);
        first_active_skill_functions.insert(127, Box::new(NoneFunction) as Box<dyn GameFirstActiveSkillHandler>);
        first_active_skill_functions.insert(128, Box::new(NoneFunction) as Box<dyn GameFirstActiveSkillHandler>);
        first_active_skill_functions.insert(130, Box::new(NoneFunction) as Box<dyn GameFirstActiveSkillHandler>);
        first_active_skill_functions.insert(133, Box::new(NoneFunction) as Box<dyn GameFirstActiveSkillHandler>);
        first_active_skill_functions.insert(134, Box::new(NoneFunction) as Box<dyn GameFirstActiveSkillHandler>);
        first_active_skill_functions.insert(136, Box::new(NoneFunction) as Box<dyn GameFirstActiveSkillHandler>);
        first_active_skill_functions.insert(139, Box::new(NoneFunction) as Box<dyn GameFirstActiveSkillHandler>);
        first_active_skill_functions.insert(140, Box::new(NoneFunction) as Box<dyn GameFirstActiveSkillHandler>);
        first_active_skill_functions.insert(160, Box::new(NoneFunction) as Box<dyn GameFirstActiveSkillHandler>);
        first_active_skill_functions.insert(161, Box::new(NoneFunction) as Box<dyn GameFirstActiveSkillHandler>);
        first_active_skill_functions.insert(162, Box::new(NoneFunction) as Box<dyn GameFirstActiveSkillHandler>);
        first_active_skill_functions.insert(164, Box::new(NoneFunction) as Box<dyn GameFirstActiveSkillHandler>);
        first_active_skill_functions.insert(171, Box::new(NoneFunction) as Box<dyn GameFirstActiveSkillHandler>);
        first_active_skill_functions.insert(173, Box::new(NoneFunction) as Box<dyn GameFirstActiveSkillHandler>);
        first_active_skill_functions.insert(174, Box::new(NoneFunction) as Box<dyn GameFirstActiveSkillHandler>);
        first_active_skill_functions.insert(176, Box::new(NoneFunction) as Box<dyn GameFirstActiveSkillHandler>);
        first_active_skill_functions.insert(177, Box::new(NoneFunction) as Box<dyn GameFirstActiveSkillHandler>);
        first_active_skill_functions.insert(178, Box::new(NoneFunction) as Box<dyn GameFirstActiveSkillHandler>);
        first_active_skill_functions.insert(179, Box::new(NoneFunction) as Box<dyn GameFirstActiveSkillHandler>);
        first_active_skill_functions.insert(183, Box::new(NoneFunction) as Box<dyn GameFirstActiveSkillHandler>);
        first_active_skill_functions.insert(184, Box::new(NoneFunction) as Box<dyn GameFirstActiveSkillHandler>);
        first_active_skill_functions.insert(187, Box::new(NoneFunction) as Box<dyn GameFirstActiveSkillHandler>);
        first_active_skill_functions.insert(189, Box::new(NoneFunction) as Box<dyn GameFirstActiveSkillHandler>);
        first_active_skill_functions.insert(191, Box::new(NoneFunction) as Box<dyn GameFirstActiveSkillHandler>);
        first_active_skill_functions.insert(193, Box::new(NoneFunction) as Box<dyn GameFirstActiveSkillHandler>);
        first_active_skill_functions.insert(198, Box::new(NoneFunction) as Box<dyn GameFirstActiveSkillHandler>);
        first_active_skill_functions.insert(200, Box::new(NoneFunction) as Box<dyn GameFirstActiveSkillHandler>);
        first_active_skill_functions.insert(201, Box::new(NoneFunction) as Box<dyn GameFirstActiveSkillHandler>);
        first_active_skill_functions.insert(202, Box::new(NoneFunction) as Box<dyn GameFirstActiveSkillHandler>);

        GameFirstActiveSkillRepositoryImpl { first_active_skill_functions }
    }

    fn get_function(&self, number: i32) -> Option<&Box<dyn GameFirstActiveSkillHandler>> {
        self.first_active_skill_functions.get(&number)
    }
}

impl GameFirstActiveSkillRepository for GameFirstActiveSkillRepositoryImpl {
    unsafe fn call_first_active_skill_repository_table(&self, use_first_active_skill_request: UseFirstActiveSkillRequest) -> UseFirstActiveSkillResponse {
        println!("GameFirstActiveSkillRepositoryImpl: call_first_active_skill_repository_table()");

        UseFirstActiveSkillResponse
    }
}

#[cfg(test)]
mod tests {
    use std::io;
    use std::io::Write;
    use super::*;

    #[test]
    fn test_game_first_active_repository_impl() {
        let repository = GameFirstActiveSkillRepositoryImpl::new();

        let number1 = 6;
        let function1 = repository.get_function(number1);
        assert!(function1.is_some());

        let response1 = unsafe { function1.unwrap().use_first_active_skill(UseFirstActiveSkillRequest) };
        assert_eq!(response1, UseFirstActiveSkillResponse);
    }

    #[test]
    fn test_none_function() {
        let mut output = Vec::new();
        let mut capture = io::Cursor::new(&mut output);
        writeln!(capture, "아직 구현되지 않은 기능입니다.").unwrap();

        let none_function = NoneFunction;
        let request = UseFirstActiveSkillRequest;
        unsafe { none_function.use_first_active_skill(request); }

        let captured_output = String::from_utf8(output.clone()).unwrap();
        assert_eq!(captured_output.trim(), "아직 구현되지 않은 기능입니다.");
    }
}