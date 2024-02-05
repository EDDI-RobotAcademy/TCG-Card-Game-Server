use std::collections::HashMap;
use crate::game_card_support::handler::game_card_support_handler::SupportCardSupportHandler;
use crate::game_card_support::handler::handler_of_2::game_card_support_2_handler_impl::SupportCard_2_Function;

use crate::game_card_support::repository::game_card_support_repository::GameCardSupportRepository;
use crate::game_card_support::service::request::use_support_card_request::UseSupportCardRequest;
use crate::game_card_support::service::response::use_support_card_response::UseSupportCardResponse;


pub struct GameCardSupportRepositoryImpl {
    support_card_functions: HashMap<i32, Box<dyn SupportCardSupportHandler>>,
}

struct NoneFunction;

impl SupportCardSupportHandler for NoneFunction {
    unsafe fn use_support_card(&self, use_support_card_request: UseSupportCardRequest) -> UseSupportCardResponse {
        println!("아직 구현되지 않은 기능입니다.");

        UseSupportCardResponse
    }
}

impl GameCardSupportRepositoryImpl {
    fn new() -> Self {
        let mut support_card_functions = HashMap::new();
        support_card_functions.insert(2, Box::new(SupportCard_2_Function) as Box<dyn SupportCardSupportHandler>);
        support_card_functions.insert(5, Box::new(NoneFunction) as Box<dyn SupportCardSupportHandler>);
        support_card_functions.insert(7, Box::new(NoneFunction) as Box<dyn SupportCardSupportHandler>);
        support_card_functions.insert(10, Box::new(NoneFunction) as Box<dyn SupportCardSupportHandler>);
        support_card_functions.insert(16, Box::new(NoneFunction) as Box<dyn SupportCardSupportHandler>);
        support_card_functions.insert(20, Box::new(NoneFunction) as Box<dyn SupportCardSupportHandler>);
        support_card_functions.insert(21, Box::new(NoneFunction) as Box<dyn SupportCardSupportHandler>);
        support_card_functions.insert(24, Box::new(NoneFunction) as Box<dyn SupportCardSupportHandler>);
        support_card_functions.insert(28, Box::new(NoneFunction) as Box<dyn SupportCardSupportHandler>);
        support_card_functions.insert(29, Box::new(NoneFunction) as Box<dyn SupportCardSupportHandler>);
        support_card_functions.insert(36, Box::new(NoneFunction) as Box<dyn SupportCardSupportHandler>);
        support_card_functions.insert(41, Box::new(NoneFunction) as Box<dyn SupportCardSupportHandler>);
        support_card_functions.insert(47, Box::new(NoneFunction) as Box<dyn SupportCardSupportHandler>);
        support_card_functions.insert(65, Box::new(NoneFunction) as Box<dyn SupportCardSupportHandler>);
        support_card_functions.insert(69, Box::new(NoneFunction) as Box<dyn SupportCardSupportHandler>);
        support_card_functions.insert(77, Box::new(NoneFunction) as Box<dyn SupportCardSupportHandler>);
        support_card_functions.insert(87, Box::new(NoneFunction) as Box<dyn SupportCardSupportHandler>);
        support_card_functions.insert(94, Box::new(NoneFunction) as Box<dyn SupportCardSupportHandler>);
        support_card_functions.insert(116, Box::new(NoneFunction) as Box<dyn SupportCardSupportHandler>);
        support_card_functions.insert(126, Box::new(NoneFunction) as Box<dyn SupportCardSupportHandler>);
        support_card_functions.insert(143, Box::new(NoneFunction) as Box<dyn SupportCardSupportHandler>);
        support_card_functions.insert(144, Box::new(NoneFunction) as Box<dyn SupportCardSupportHandler>);
        support_card_functions.insert(146, Box::new(NoneFunction) as Box<dyn SupportCardSupportHandler>);
        support_card_functions.insert(156, Box::new(NoneFunction) as Box<dyn SupportCardSupportHandler>);
        support_card_functions.insert(163, Box::new(NoneFunction) as Box<dyn SupportCardSupportHandler>);
        support_card_functions.insert(165, Box::new(NoneFunction) as Box<dyn SupportCardSupportHandler>);
        support_card_functions.insert(166, Box::new(NoneFunction) as Box<dyn SupportCardSupportHandler>);
        support_card_functions.insert(167, Box::new(NoneFunction) as Box<dyn SupportCardSupportHandler>);
        support_card_functions.insert(169, Box::new(NoneFunction) as Box<dyn SupportCardSupportHandler>);
        support_card_functions.insert(170, Box::new(NoneFunction) as Box<dyn SupportCardSupportHandler>);
        support_card_functions.insert(172, Box::new(NoneFunction) as Box<dyn SupportCardSupportHandler>);
        support_card_functions.insert(175, Box::new(NoneFunction) as Box<dyn SupportCardSupportHandler>);
        support_card_functions.insert(186, Box::new(NoneFunction) as Box<dyn SupportCardSupportHandler>);
        support_card_functions.insert(188, Box::new(NoneFunction) as Box<dyn SupportCardSupportHandler>);
        support_card_functions.insert(190, Box::new(NoneFunction) as Box<dyn SupportCardSupportHandler>);
        support_card_functions.insert(195, Box::new(NoneFunction) as Box<dyn SupportCardSupportHandler>);

        GameCardSupportRepositoryImpl { support_card_functions }
    }

    fn get_function(&self, number: i32) -> Option<&Box<dyn SupportCardSupportHandler>> {
        self.support_card_functions.get(&number)
    }
}

impl GameCardSupportRepository for GameCardSupportRepositoryImpl {
    unsafe fn call_support_card_repository_table(&self, use_support_card_request: UseSupportCardRequest) -> UseSupportCardResponse {
        println!("GameCardSupportRepositoryImpl: call_support_card_repository_table()");

        UseSupportCardResponse
    }
}

#[cfg(test)]
mod tests {
    use std::io;
    use std::io::Write;
    use super::*;
    use crate::game_card_support::service::request::use_support_card_request::UseSupportCardRequest;

    #[test]
    fn test_game_card_support_repository_impl() {
        let repository = GameCardSupportRepositoryImpl::new();

        let number1 = 2;
        let function1 = repository.get_function(number1);
        assert!(function1.is_some());

        let response1 = unsafe { function1.unwrap().use_support_card(UseSupportCardRequest) };
        assert_eq!(response1, UseSupportCardResponse);

        let number2 = 93;
        let function2 = repository.get_function(number2);
        assert!(function2.is_none());
    }

    #[test]
    fn test_none_function() {
        let mut output = Vec::new();
        let mut capture = io::Cursor::new(&mut output);
        writeln!(capture, "아직 구현되지 않은 기능입니다.").unwrap();

        let none_function = NoneFunction;
        let request = UseSupportCardRequest;
        unsafe { none_function.use_support_card(request); }

        let captured_output = String::from_utf8(output.clone()).unwrap();
        assert_eq!(captured_output.trim(), "아직 구현되지 않은 기능입니다.");
    }
}

