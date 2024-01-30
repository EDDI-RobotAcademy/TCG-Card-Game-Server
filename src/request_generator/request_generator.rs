use serde_json::Value as JsonValue;
use crate::account::service::account_service::AccountService;
use crate::account::service::account_service_impl::AccountServiceImpl;
use crate::account_deck::service::account_deck_service::AccountDeckService;
use crate::account_deck::service::account_deck_service_impl::AccountDeckServiceImpl;
use crate::battle_ready_monitor::service::battle_ready_monitor_service::BattleReadyMonitorService;
use crate::battle_ready_monitor::service::battle_ready_monitor_service_impl::BattleReadyMonitorServiceImpl;
use crate::battle_room::service::battle_room_service::BattleRoomService;
use crate::battle_room::service::battle_room_service_impl::BattleRoomServiceImpl;
use crate::client_program::service::client_program_service::ClientProgramService;
use crate::client_program::service::client_program_service_impl::ClientProgramServiceImpl;
use crate::deck_card::service::deck_card_service::DeckCardService;
use crate::deck_card::service::deck_card_service_impl::DeckCardServiceImpl;
use crate::request_generator::account_deck_request_generator::{create_deck_list_request, create_deck_modify_request, create_deck_register_request};
use crate::request_generator::account_request_generator::{create_account_delete_request, create_account_modify_request, create_login_request, create_logout_request, create_register_request};
use crate::request_generator::battle_ready_request_generator::create_battle_ready_request;
use crate::request_generator::battle_match_request_generator::create_battle_match_request;
use crate::request_generator::client_program_request_generator::create_client_program_exit_request;
use crate::request_generator::deck_card_request_generator::{create_deck_card_list_request, create_deck_configuration_request};
use crate::request_generator::session_request_generator::create_session_login_request;
use crate::response_generator::response_type::ResponseType;


// TODO: 이 부분도 같이 ugly 해졌는데 추후 고칠 필요 있음
pub async fn create_request_and_call_service(data: &JsonValue) -> Option<ResponseType> {
    println!("protocol 번호 분석");
    if let Some(protocol_number) = data.get("protocolNumber").and_then(|v| v.as_i64()) {
        // TODO: 이 부분 전부 번호에 따라 동작하도록 Table 처리가 필요함
        match protocol_number {
            1 => {
                // Account Register
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
                // Account Login
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
                // Account Session Login
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
            4 => {
                // Account Logout
                if let Some(request) = create_logout_request(&data) {
                    let account_service_mutex = AccountServiceImpl::get_instance();
                    let mut account_service = account_service_mutex.lock().await;

                    let response = account_service.account_logout(request).await;
                    let response_type = Some(ResponseType::ACCOUNT_LOGOUT(response));

                    response_type
                } else {
                    None
                }
            },
            5 => {
                // Account Modify
                if let Some(request) = create_account_modify_request(&data) {
                    let account_service_mutex = AccountServiceImpl::get_instance();
                    let mut account_service = account_service_mutex.lock().await;

                    let response = account_service.account_modify(request).await;
                    let response_type = Some(ResponseType::ACCOUNT_MODIFY(response));

                    response_type
                } else {
                    None
                }
            }
            6 => {
                // Account Delete
                if let Some(request) = create_account_delete_request(&data) {
                    let account_service_mutex = AccountServiceImpl::get_instance();
                    let mut account_service = account_service_mutex.lock().await;

                    let response = account_service.account_delete(request).await;
                    let response_type = Some(ResponseType::ACCOUNT_DELETE(response));

                    response_type
                } else {
                    None
                }
            }
            11 => {
                // Battle Deck List
                if let Some(request) = create_deck_list_request(&data) {
                    let account_deck_service_mutex = AccountDeckServiceImpl::get_instance();
                    let mut account_deck_service = account_deck_service_mutex.lock().await;

                    let response = account_deck_service.account_deck_list(request).await;
                    let response_type = Some(ResponseType::BATTLE_DECK_LIST(response));

                    response_type
                } else {
                    None
                }
            },
            12 => {
                // Battle Match
                if let Some(request) = create_battle_match_request(&data) {
                    println!("request generator: battle match request protocol");
                    let battle_room_service_mutex = BattleRoomServiceImpl::get_instance();
                    let mut battle_room_service = battle_room_service_mutex.lock().await;

                    let response = battle_room_service.enqueue_player_id_to_wait_queue(request).await;
                    let response_type = Some(ResponseType::BATTLE_MATCH(response));
                    println!("response_type: {:?}", response_type);

                    response_type
                } else {
                    None
                }
            },
            13 => {
                // Is Ready For Battle
                if let Some(request) = create_battle_ready_request(&data) {
                    let battle_ready_service_mutex = BattleReadyMonitorServiceImpl::get_instance();
                    let mut battle_ready_service = battle_ready_service_mutex.lock().await;

                    let response = battle_ready_service.check_ready_for_battle(request).await;
                    let response_type = Some(ResponseType::BATTLE_READY(response));

                    response_type
                } else {
                    None
                }
            },
            14 => {
                // Battle Deck Card List
                if let Some(request) = create_deck_card_list_request(&data) {
                    let deck_card_service_mutex = DeckCardServiceImpl::get_instance();
                    let mut deck_card_service = deck_card_service_mutex.lock().await;

                    let response = deck_card_service.deck_card_list(request).await;
                    let response_type = Some(ResponseType::BATTLE_DECK_CARD_LIST(response));

                    response_type
                } else {
                    None
                }
            },
            41 => {
                // Account Deck Register
                if let Some(request) = create_deck_register_request(&data) {
                    let account_deck_service_mutex = AccountDeckServiceImpl::get_instance();
                    let mut account_deck_service = account_deck_service_mutex.lock().await;

                    let response = account_deck_service.account_deck_register(request).await;
                    let response_type = Some(ResponseType::ACCOUNT_DECK_REGISTER(response));

                    response_type
                } else {
                    None
                }
            },
            42 => {
                // Account Deck (Name) List
                if let Some(request) = create_deck_list_request(&data) {
                    let account_deck_service_mutex = AccountDeckServiceImpl::get_instance();
                    let mut account_deck_service = account_deck_service_mutex.lock().await;

                    let response = account_deck_service.account_deck_list(request).await;
                    let response_type = Some(ResponseType::ACCOUNT_DECK_LIST(response));

                    response_type
                } else {
                    None
                }
            },
            43 => {
                // Account Deck Modify
                if let Some(request) = create_deck_modify_request(&data) {
                    let account_deck_service_mutex = AccountDeckServiceImpl::get_instance();
                    let mut account_deck_service = account_deck_service_mutex.lock().await;

                    let response = account_deck_service.account_deck_modify(request).await;
                    let response_type = Some(ResponseType::ACCOUNT_DECK_MODIFY(response));

                    response_type
                } else {
                    None
                }
            },
            51 => {
                // Deck Card Configuration
                if let Some(request) = create_deck_configuration_request(&data) {
                    let deck_card_service_mutex = DeckCardServiceImpl::get_instance();
                    let mut deck_card_service = deck_card_service_mutex.lock().await;

                    let response = deck_card_service.deck_configuration_register(request).await;
                    let response_type = Some(ResponseType::DECK_CARD_CONFIGURATION(response));

                    response_type
                } else {
                    None
                }
            },
            52 => {
                // (Account) Deck Card List
                if let Some(request) = create_deck_card_list_request(&data) {
                    let deck_card_service_mutex = DeckCardServiceImpl::get_instance();
                    let mut deck_card_service = deck_card_service_mutex.lock().await;

                    let response = deck_card_service.deck_card_list(request).await;
                    let response_type = Some(ResponseType::DECK_CARD_LIST(response));

                    response_type
                } else {
                    None
                }
            },
            4444 => {
                // Program Exit
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