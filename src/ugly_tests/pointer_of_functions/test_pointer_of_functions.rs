use std::ffi::c_void;

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

unsafe extern "C" fn function1_impl(arg: *mut c_void) -> *mut c_void {
    println!("1번째 함수 실행");
    arg
}

unsafe extern "C" fn function2_impl(arg: *mut c_void) -> *mut c_void {
    println!("2번째 함수 실행");
    arg
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::test;

    #[test]
    async fn test_pointer_of_functions() {
        let function_table = initialize_function_table();

        unsafe {
            let argument = std::ptr::null_mut();
            let result1 = (function_table.function1)(argument);
            let result2 = (function_table.function2)(argument);
        }
    }
}
