use std::ffi::c_void;
use serde::{Deserialize, Serialize};

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

type MyFunction = unsafe extern "C" fn(*mut c_void) -> *mut c_void;

struct FunctionTable {
    function1: MyFunction,
    function2: MyFunction,
}

fn initialize_function_table() -> FunctionTable {
    FunctionTable {
        function1: function1_impl,
        function2: function2_impl,
    }
}

unsafe extern "C" fn function1_impl(request: *mut c_void) -> *mut c_void {
    let test_request = &*(request as *const TestRequest);
    let mut test_response = Box::new(TestResponse {
        number: test_request.number,
        count: test_request.count,
    });

    println!("1번째 함수 실행");

    Box::into_raw(test_response) as *mut c_void
}

unsafe extern "C" fn function2_impl(request: *mut c_void) -> *mut c_void {
    let test_request = &*(request as *const TestRequest);
    let mut test_response = Box::new(TestResponse {
        number: test_request.number,
        count: test_request.count,
    });

    println!("2번째 함수 실행");

    Box::into_raw(test_response) as *mut c_void
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::test;

    #[test]
    async fn test_pointer_of_functions() {
        let function_table = initialize_function_table();

        unsafe {
            let request = Box::new(TestRequest {
                number: 42,
                type_: "example".to_string(),
                count: 10,
            });

            let response = Box::from_raw((function_table.function1)(
                Box::into_raw(request) as *mut c_void,
            ) as *mut TestResponse);

            println!("Response: {:?}", response);
            assert_eq!(response.count, 10);
        }
    }
}
