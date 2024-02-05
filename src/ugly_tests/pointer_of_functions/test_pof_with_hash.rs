use std::collections::HashMap;
use std::ffi::c_void;

#[derive(Clone)]
struct TestRequest {
    number: i32,
    type_: String,
    count: i32,
}

#[derive(Debug)]
struct TestResponse {
    number: i32,
    count: i32,
}

trait MyFunctionTrait {
    unsafe fn call(&self, request: TestRequest) -> TestResponse;
}

struct Function1Impl;

impl MyFunctionTrait for Function1Impl {
    unsafe fn call(&self, request: TestRequest) -> TestResponse {
        let mut test_response = TestResponse {
            number: request.number,
            count: request.count,
        };

        println!("Function1Impl 실행");
        println!("Response: {:?}", test_response);

        test_response
    }
}

struct Function2Impl;

impl MyFunctionTrait for Function2Impl {
    unsafe fn call(&self, request: TestRequest) -> TestResponse {
        let mut test_response = TestResponse {
            number: request.number,
            count: request.count,
        };

        println!("Function2Impl 실행");
        println!("Response: {:?}", test_response);

        test_response
    }
}

struct Function3Impl;

impl MyFunctionTrait for Function3Impl {
    unsafe fn call(&self, request: TestRequest) -> TestResponse {
        let mut test_response = TestResponse {
            number: request.number,
            count: request.count,
        };

        println!("Function3Impl 실행");
        println!("Response: {:?}", test_response);

        test_response
    }
}

struct FunctionTable {
    functions: HashMap<i32, Box<dyn MyFunctionTrait>>,
}

impl FunctionTable {
    fn new() -> Self {
        let mut functions = HashMap::new();
        functions.insert(2, Box::new(Function1Impl) as Box<dyn MyFunctionTrait>);
        functions.insert(93, Box::new(Function2Impl) as Box<dyn MyFunctionTrait>);
        functions.insert(57, Box::new(Function3Impl) as Box<dyn MyFunctionTrait>);

        FunctionTable { functions }
    }

    fn get_function(&self, number: i32) -> Option<&Box<dyn MyFunctionTrait>> {
        self.functions.get(&number)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::test;

    #[test]
    async fn test_function_pointer_table() {
        let function_table = FunctionTable::new();

        let number1 = 2;
        let number2 = 93;
        let number3 = 57;

        if let Some(function) = function_table.get_function(number1) {
            let request = TestRequest {
                number: 42,
                type_: "example".to_string(),
                count: 10,
            };
            let response = unsafe { function.call(request.clone()) };

            assert_eq!(response.count, 10);
            println!("response: {:?}", response);
        }

        if let Some(function) = function_table.get_function(number2) {
            let request = TestRequest {
                number: 93,
                type_: "example".to_string(),
                count: 20,
            };
            let response = unsafe { function.call(request.clone()) };

            assert_eq!(response.count, 20);
            println!("response: {:?}", response);
        }

        if let Some(function) = function_table.get_function(number3) {
            let request = TestRequest {
                number: 57,
                type_: "example".to_string(),
                count: 30,
            };
            let response = unsafe { function.call(request.clone()) };

            assert_eq!(response.count, 30);
            println!("response: {:?}", response);
        }
    }
}
