use serde_json::Value as JsonValue;
use crate::account::service::request::account_register_request::AccountRegisterRequest;
use crate::request_generator::account_register_req::create_register_request;

#[derive(Debug)]
pub enum RequestType {
    ACCOUNT_REGISTER(AccountRegisterRequest),
}

trait RequestGenerator {
    fn generate(&self, value: JsonValue) -> Option<RequestType>;
}

// TODO: 이 부분도 같이 ugly 해졌는데 추후 고칠 필요 있음
pub fn create_account_request_and_call_service(data: &JsonValue) -> Option<RequestType> {
    println!("protocol 번호 분석");
    if let Some(protocol_number) = data.get("protocolNumber").and_then(|v| v.as_i64()) {
        match protocol_number {
            1 => {
                let request = create_register_request(&data).map(RequestType::ACCOUNT_REGISTER);
                println!("request: {:?}", request.unwrap());
                None

            },
            _ => None,
        }
    } else {
        None
    }
}