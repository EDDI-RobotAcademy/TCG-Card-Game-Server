use serde_json::Value as JsonValue;
use crate::account::service::account_service::AccountService;
use crate::account::service::account_service_impl::AccountServiceImpl;
use crate::account::service::request::account_register_request::AccountRegisterRequest;
use crate::client_program::service::client_program_service::ClientProgramService;
use crate::client_program::service::client_program_service_impl::ClientProgramServiceImpl;
use crate::request_generator::account_request_generator::{create_login_request, create_register_request};
use crate::request_generator::client_program_request_generator::create_client_program_exit_request;
use crate::request_generator::session_request_generator::create_session_login_request;
use crate::response_generator::response_type::ResponseType;

#[derive(Debug)]
pub enum RequestType {
    ACCOUNT_REGISTER(AccountRegisterRequest),
}

trait RequestGenerator {
    fn generate(&self, value: JsonValue) -> Option<RequestType>;
}

// TODO: 이 부분도 같이 ugly 해졌는데 추후 고칠 필요 있음
pub async fn create_request_and_call_service(data: &JsonValue) -> Option<ResponseType> {
    println!("protocol 번호 분석");
    if let Some(protocol_number) = data.get("protocolNumber").and_then(|v| v.as_i64()) {
        // TODO: 이 부분 전부 번호에 따라 동작하도록 Table 처리가 필요함
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
            2 => {
                if let Some(request) = create_login_request(&data) {
                    let account_service_mutex = AccountServiceImpl::get_instance();
                    let mut account_service = account_service_mutex.lock().await;

                    let response = account_service.account_login(request).await;
                    let response_type = Some(ResponseType::ACCOUNT_LOGIN(response));

                    response_type
                } else {
                    None
                }
            },
            3 => {
                if let Some(request) = create_session_login_request(&data) {
                    let account_service_mutex = AccountServiceImpl::get_instance();
                    let mut account_service = account_service_mutex.lock().await;

                    let response = account_service.account_session_login(request).await;
                    let response_type = Some(ResponseType::ACCOUNT_LOGIN(response));

                    response_type
                } else {
                    None
                }
            },
            11 => {
                // Account Deck info 를 요청한 것이므로 이에 대해 응답해야함 (Select Account Deck for Battle)
                None
            },
            101 => {
                // Request Battle Match
                None
            },
            102 => {
                // Is Ready For Battle
                None
            },
            4444 => {
                if let Some(request) = create_client_program_exit_request(&data) {
                    let client_program_service_mutex = ClientProgramServiceImpl::get_instance();
                    let mut client_program_service = client_program_service_mutex.lock().await;

                    let response = client_program_service.client_exit_program(request).await;
                    let response_type = Some(ResponseType::PROGRAM_EXIT(response));

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