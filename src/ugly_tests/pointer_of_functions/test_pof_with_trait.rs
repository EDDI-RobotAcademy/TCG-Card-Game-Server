use std::ffi::c_void;

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

// trait 정의
trait MyFunctionTrait {
    unsafe fn call(&self, request: TestRequest) -> TestResponse;
}

// 구현체
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

// 함수 포인터를 감싸는 enum
enum FunctionPointer {
    Function1(Box<dyn MyFunctionTrait>),
    Function2(Box<dyn MyFunctionTrait>),
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::test;

    #[test]
    async fn test_function_pointer_trait() {
        let function1 = FunctionPointer::Function1(Box::new(Function1Impl));
        let function2 = FunctionPointer::Function2(Box::new(Function2Impl));

        let request = TestRequest {
            number: 42,
            type_: "example".to_string(),
            count: 10,
        };

        match function1 {
            FunctionPointer::Function1(f) => unsafe {
                let response = f.call(request);
                assert_eq!(response.count, 10);
            }
            _ => {}
        }

        let request = TestRequest {
            number: 42,
            type_: "example".to_string(),
            count: 10,
        };

        match function2 {
            FunctionPointer::Function2(f) => unsafe {
                let response = f.call(request);
                assert_eq!(response.count, 10);
            }
            _ => {}
        }
    }
}
