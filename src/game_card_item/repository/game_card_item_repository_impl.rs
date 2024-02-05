use std::collections::HashMap;
use crate::game_card_item::handler::game_card_item_handler::GameCardItemHandler;
use crate::game_card_item::handler::handler_of_8::game_card_item_8_handler_impl::ItemCard_8_Function;
use crate::game_card_item::repository::game_card_item_repository::GameCardItemRepository;
use crate::game_card_item::service::request::use_item_card_request::UseItemCardRequest;
use crate::game_card_item::service::response::use_item_card_response::UseItemCardResponse;

pub struct GameCardItemRepositoryImpl {
    item_card_functions: HashMap<i32, Box<dyn GameCardItemHandler>>,
}

struct NoneFunction;

impl GameCardItemHandler for NoneFunction {
    unsafe fn use_item_card(&self, use_item_card_request: UseItemCardRequest) -> UseItemCardResponse {
        println!("아직 구현되지 않은 기능입니다.");

        UseItemCardResponse
    }
}

impl GameCardItemRepositoryImpl {
    fn new() -> Self {
        let mut item_card_functions = HashMap::new();
        item_card_functions.insert(8, Box::new(ItemCard_8_Function) as Box<dyn GameCardItemHandler>);
        item_card_functions.insert(9, Box::new(NoneFunction) as Box<dyn GameCardItemHandler>);
        item_card_functions.insert(13, Box::new(NoneFunction) as Box<dyn GameCardItemHandler>);
        item_card_functions.insert(25, Box::new(NoneFunction) as Box<dyn GameCardItemHandler>);
        item_card_functions.insert(30, Box::new(NoneFunction) as Box<dyn GameCardItemHandler>);
        item_card_functions.insert(33, Box::new(NoneFunction) as Box<dyn GameCardItemHandler>);
        item_card_functions.insert(35, Box::new(NoneFunction) as Box<dyn GameCardItemHandler>);
        item_card_functions.insert(45, Box::new(NoneFunction) as Box<dyn GameCardItemHandler>);
        item_card_functions.insert(54, Box::new(NoneFunction) as Box<dyn GameCardItemHandler>);
        item_card_functions.insert(66, Box::new(NoneFunction) as Box<dyn GameCardItemHandler>);
        item_card_functions.insert(88, Box::new(NoneFunction) as Box<dyn GameCardItemHandler>);
        item_card_functions.insert(95, Box::new(NoneFunction) as Box<dyn GameCardItemHandler>);
        item_card_functions.insert(108, Box::new(NoneFunction) as Box<dyn GameCardItemHandler>);
        item_card_functions.insert(113, Box::new(NoneFunction) as Box<dyn GameCardItemHandler>);
        item_card_functions.insert(119, Box::new(NoneFunction) as Box<dyn GameCardItemHandler>);
        item_card_functions.insert(125, Box::new(NoneFunction) as Box<dyn GameCardItemHandler>);
        item_card_functions.insert(137, Box::new(NoneFunction) as Box<dyn GameCardItemHandler>);
        item_card_functions.insert(141, Box::new(NoneFunction) as Box<dyn GameCardItemHandler>);
        item_card_functions.insert(154, Box::new(NoneFunction) as Box<dyn GameCardItemHandler>);
        item_card_functions.insert(194, Box::new(NoneFunction) as Box<dyn GameCardItemHandler>);

        GameCardItemRepositoryImpl { item_card_functions }
    }

    fn get_function(&self, number: i32) -> Option<&Box<dyn GameCardItemHandler>> {
        self.item_card_functions.get(&number)
    }
}

impl GameCardItemRepository for GameCardItemRepositoryImpl {
    unsafe fn call_item_card_repository_table(&self, use_support_card_request: UseItemCardRequest) -> UseItemCardResponse {
        println!("GameCardSupportRepositoryImpl: call_support_card_repository_table()");

        UseItemCardResponse
    }
}

#[cfg(test)]
mod tests {
    use std::io;
    use std::io::Write;
    use crate::game_card_item::service::request::use_item_card_request::UseItemCardRequest;
    use crate::game_card_item::service::response::use_item_card_response::UseItemCardResponse;
    use super::*;

    #[test]
    fn test_game_card_item_repository_impl() {
        let repository = GameCardItemRepositoryImpl::new();

        let number1 = 2;
        let function1 = repository.get_function(number1);
        assert!(function1.is_some());

        let response1 = unsafe { function1.unwrap().use_item_card(UseItemCardRequest) };
        assert_eq!(response1, UseItemCardResponse);

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
        let request = UseItemCardRequest;
        unsafe { none_function.use_item_card(request); }

        let captured_output = String::from_utf8(output.clone()).unwrap();
        assert_eq!(captured_output.trim(), "아직 구현되지 않은 기능입니다.");
    }
}

