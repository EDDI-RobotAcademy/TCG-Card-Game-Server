use serde_json::Value as JsonValue;
use crate::account::service::account_service::AccountService;
use crate::account::service::account_service_impl::AccountServiceImpl;
use crate::account_deck::service::account_deck_service::AccountDeckService;
use crate::account_deck::service::account_deck_service_impl::AccountDeckServiceImpl;
use crate::account_point::service::account_point_service::AccountPointService;
use crate::account_point::service::account_point_service_impl::AccountPointServiceImpl;
use crate::battle_ready_account_hash::service::battle_ready_account_hash_service::BattleReadyAccountHashService;
use crate::battle_ready_account_hash::service::battle_ready_account_hash_service_impl::BattleReadyAccountHashServiceImpl;
use crate::battle_room::service::battle_room_service::BattleRoomService;
use crate::battle_room::service::battle_room_service_impl::BattleRoomServiceImpl;

use crate::battle_wait_queue::service::battle_wait_queue_service::BattleWaitQueueService;
use crate::battle_wait_queue::service::battle_wait_queue_service_impl::BattleWaitQueueServiceImpl;
use crate::client_program::service::client_program_service::ClientProgramService;
use crate::client_program::service::client_program_service_impl::ClientProgramServiceImpl;
use crate::account_deck_card::service::account_deck_card_service::AccountDeckCardService;
use crate::account_deck_card::service::account_deck_card_service_impl::AccountDeckCardServiceImpl;
use crate::request_generator::account_deck_request_generator::{create_deck_list_request, create_deck_modify_request, create_deck_register_request};
use crate::request_generator::account_point_request_generator::{create_gain_gold_request, create_pay_gold_request};
use crate::request_generator::account_request_generator::{create_account_delete_request, create_account_modify_request, create_login_request, create_logout_request, create_register_request};
use crate::request_generator::battle_ready_account_hash_request_generator::create_battle_ready_account_hash_request;
use crate::request_generator::battle_wait_queue_request_generator::create_battle_wait_queue_request;
use crate::request_generator::check_battle_prepare_request_generator::create_check_battle_prepare_request;
use crate::request_generator::client_program_request_generator::create_client_program_exit_request;
use crate::request_generator::deck_card_request_generator::{create_deck_card_list_request, create_deck_configuration_request};
use crate::request_generator::game_deck_card_list_request_generator::create_game_deck_card_list_request;
use crate::request_generator::session_request_generator::create_session_login_request;
use crate::request_generator::shop_request_generator::{create_free_card_request, create_get_card_default_request};
use crate::request_generator::what_is_the_room_number_request_generator::create_what_is_the_room_number_request;
use crate::response_generator::response_type::ResponseType;
use crate::shop::service::shop_service::ShopService;
use crate::shop::service::shop_service_impl::ShopServiceImpl;


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
                // Battle Wait Queue for Match
                if let Some(request) = create_battle_wait_queue_request(&data) {
                    println!("request generator: battle match request protocol");
                    let battle_wait_queue_service_mutex = BattleWaitQueueServiceImpl::get_instance();
                    let mut battle_wait_queue_service = battle_wait_queue_service_mutex.lock().await;

                    let response = battle_wait_queue_service.enqueue_player_id_to_wait_queue(request).await;
                    let response_type = Some(ResponseType::BATTLE_WAIT_QUEUE_FOR_MATCH(response));
                    println!("response_type: {:?}", response_type);

                    response_type
                } else {
                    None
                }
            },
            12 => {
                // Is Ready For Battle
                if let Some(request) = create_battle_ready_account_hash_request(&data) {
                    let battle_ready_account_hash_service_mutex = BattleReadyAccountHashServiceImpl::get_instance();
                    let mut battle_ready_account_hash_service_guard = battle_ready_account_hash_service_mutex.lock().await;

                    let response = battle_ready_account_hash_service_guard.check_ready_for_battle(request).await;
                    let response_type = Some(ResponseType::BATTLE_READY(response));

                    response_type
                } else {
                    None
                }
            },
            13 => {
                // Battle Match Cancel
                None
            },
            14 => {
                // Check Battle Prepare (CHECK_BATTLE_PREPARE)
                if let Some(request) = create_check_battle_prepare_request(&data) {
                    let battle_ready_account_hash_service_mutex = BattleReadyAccountHashServiceImpl::get_instance();
                    let mut battle_ready_account_hash_service_guard = battle_ready_account_hash_service_mutex.lock().await;

                    let response = battle_ready_account_hash_service_guard.check_prepare_for_battle(request).await;
                    let response_type = Some(ResponseType::CHECK_BATTLE_PREPARE(response));

                    response_type
                } else {
                    None
                }
            },
            15 => {
                // WHAT_IS_THE_ROOM_NUMBER
                if let Some(request) = create_what_is_the_room_number_request(&data) {
                    let battle_room_service_mutex = BattleRoomServiceImpl::get_instance();
                    let mut battle_room_service_guard = battle_room_service_mutex.lock().await;

                    let response = battle_room_service_guard.what_is_the_room_number(request).await;
                    let response_type = Some(ResponseType::WHAT_IS_THE_ROOM_NUMBER(response));

                    response_type
                } else {
                    None
                }
            },
            16 => {
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
            // 17 => {
            //     // Game Deck Card List
            //     if let Some(request) = create_game_deck_card_list_request(&data) {
            //         let game_deck_card_service_mutex = GameDeckServiceImpl::get_instance();
            //         let mut game_deck_card_service = game_deck_card_service_mutex.lock().await;
            //
            //         let response = game_deck_card_service.create_and_shuffle_deck(request).await;
            //         let response_type = Some(ResponseType::BATTLE_GAME_DECK_CARD_LIST(response));
            //
            //         response_type
            //     } else {
            //         None
            //     }
            // },
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
                    let deck_card_service_mutex = AccountDeckCardServiceImpl::get_instance();
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
                    let deck_card_service_mutex = AccountDeckCardServiceImpl::get_instance();
                    let mut deck_card_service = deck_card_service_mutex.lock().await;

                    let response = deck_card_service.deck_card_list(request).await;
                    let response_type = Some(ResponseType::DECK_CARD_LIST(response));

                    response_type
                } else {
                    None
                }
            },
            71 => {
                // Shop Get Card Default
                if let Some(request) = create_get_card_default_request(&data) {
                    let shop_service_mutex = ShopServiceImpl::get_instance();
                    let mut shop_service = shop_service_mutex.lock().await;

                    let response = shop_service.get_card_default(request).await;
                    let response_type = Some(ResponseType::SHOP_GET_CARD_DEFAULT(response));

                    response_type
                } else {
                    None
                }
            },
            91 => {
                // Gain gold
                if let Some(request) = create_gain_gold_request(&data) {
                    let account_point_service_mutex = AccountPointServiceImpl::get_instance();
                    let mut account_point_service = account_point_service_mutex.lock().await;

                    let response = account_point_service.gain_gold(request).await;
                    let response_type = Some(ResponseType::GAIN_GOLD(response));

                    response_type
                } else {
                    None
                }
            },
            92 => {
                // Pay gold
                if let Some(request) = create_pay_gold_request(&data) {
                    let account_point_service_mutex = AccountPointServiceImpl::get_instance();
                    let mut account_point_service = account_point_service_mutex.lock().await;

                    let response = account_point_service.pay_gold(request).await;
                    let response_type = Some(ResponseType::PAY_GOLD(response));

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