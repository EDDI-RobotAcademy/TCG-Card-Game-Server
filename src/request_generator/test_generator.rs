use serde_json::Value as JsonValue;
use crate::account::service::account_service::AccountService;
use crate::account::service::account_service_impl::AccountServiceImpl;
use crate::account::service::request::account_register_request::AccountRegisterRequest;
use crate::request_generator::account_register_req::create_register_request;
use crate::response_generator::response_type::ResponseType;

#[derive(Debug)]
pub enum RequestType {
    ACCOUNT_REGISTER(AccountRegisterRequest),
}

trait RequestGenerator {
    fn generate(&self, value: JsonValue) -> Option<RequestType>;
}

// TODO: 이 부분도 같이 ugly 해졌는데 추후 고칠 필요 있음
pub async fn create_account_request_and_call_service(data: &JsonValue) -> Option<ResponseType> {
    println!("protocol 번호 분석");
    if let Some(protocol_number) = data.get("protocolNumber").and_then(|v| v.as_i64()) {
        match protocol_number {
            1 => {
                if let Some(request) = create_register_request(&data) {
                    let account_service_mutex = AccountServiceImpl::get_instance();
                    let mut account_service = account_service_mutex.lock().await;

                    let response = account_service.account_register(request).await;
                    let response_type = Some(ResponseType::ACCOUNT_REGISTER(response));

                    response_type
                } else {
                    None
                }
            },
            _ => None,
        }
    } else {
        None
    }
}