use serde_json::Value as JsonValue;
use crate::account::service::account_service::AccountService;
use crate::account::service::account_service_impl::AccountServiceImpl;
use crate::account_card::service::account_card_service::AccountCardService;
use crate::account_card::service::account_card_service_impl::AccountCardServiceImpl;
use crate::account_deck::service::account_deck_service::AccountDeckService;
use crate::account_deck::service::account_deck_service_impl::AccountDeckServiceImpl;
use crate::account_deck_card::controller::account_deck_card_controller::AccountDeckCardController;
use crate::account_deck_card::controller::account_deck_card_controller_impl::AccountDeckCardControllerImpl;
use crate::account_point::service::account_point_service::AccountPointService;
use crate::account_point::service::account_point_service_impl::AccountPointServiceImpl;
use crate::battle_field_info::service::battle_field_info_service::BattleFieldInfoService;
use crate::battle_field_info::service::battle_field_info_service_impl::BattleFieldInfoServiceImpl;
use crate::battle_finish::service::battle_finish_service::BattleFinishService;
use crate::battle_ready_account_hash::service::battle_ready_account_hash_service::BattleReadyAccountHashService;
use crate::battle_ready_account_hash::service::battle_ready_account_hash_service_impl::BattleReadyAccountHashServiceImpl;
use crate::battle_room::service::battle_room_service::BattleRoomService;
use crate::battle_room::service::battle_room_service_impl::BattleRoomServiceImpl;
use crate::battle_finish::service::battle_finish_service_impl::BattleFinishServiceImpl;
use crate::battle_start::controller::battle_start_controller::BattleStartController;
use crate::battle_start::controller::battle_start_controller_impl::BattleStartControllerImpl;
use crate::battle_wait_queue::service::battle_wait_queue_service::BattleWaitQueueService;
use crate::battle_wait_queue::service::battle_wait_queue_service_impl::BattleWaitQueueServiceImpl;
use crate::check_connecting::service::check_connecting_service::CheckConnectingService;
use crate::check_connecting::service::check_connecting_service_impl::CheckConnectingServiceImpl;
use crate::client_program::service::client_program_service::ClientProgramService;
use crate::client_program::service::client_program_service_impl::ClientProgramServiceImpl;
use crate::fake_battle_room::controller::fake_battle_room_controller::FakeBattleRoomController;
use crate::fake_battle_room::controller::fake_battle_room_controller_impl::FakeBattleRoomControllerImpl;
use crate::game_card_active_skill::controller::game_card_active_skill_controller::GameCardActiveSkillController;
use crate::game_card_active_skill::controller::game_card_active_skill_controller_impl::GameCardActiveSkillControllerImpl;
use crate::game_card_energy::controller::game_card_energy_controller::GameCardEnergyController;
use crate::game_card_energy::controller::game_card_energy_controller_impl::GameCardEnergyControllerImpl;
use crate::game_card_item::controller::game_card_item_controller::GameCardItemController;
use crate::game_card_item::controller::game_card_item_controller_impl::GameCardItemControllerImpl;
use crate::game_card_passive_skill::controller::game_card_passive_skill_controller::GameCardPassiveSkillController;
use crate::game_card_passive_skill::controller::game_card_passive_skill_controller_impl::GameCardPassiveSkillControllerImpl;
use crate::game_card_support::controller::game_card_support_controller::GameCardSupportController;
use crate::game_card_support::controller::game_card_support_controller_impl::GameCardSupportControllerImpl;
use crate::game_card_unit::controller::game_card_unit_controller::GameCardUnitController;
use crate::game_card_unit::controller::game_card_unit_controller_impl::GameCardUnitControllerImpl;
use crate::game_deck::service::game_deck_service::GameDeckService;
use crate::game_deck::service::game_deck_service_impl::GameDeckServiceImpl;
use crate::game_field_energy::controller::game_field_energy_controller::GameFieldEnergyController;
use crate::game_field_energy::controller::game_field_energy_controller_impl::GameFieldEnergyControllerImpl;
use crate::game_turn::controller::game_turn_controller_impl::GameTurnControllerImpl;
use crate::request_generator::account_card_request_generator::create_account_card_list_request;
use crate::request_generator::account_deck_request_generator::{create_deck_delete_request, create_deck_list_request, create_deck_modify_request, create_deck_register_request};
use crate::request_generator::account_point_request_generator::{create_gain_gold_request, create_pay_gold_request};
use crate::request_generator::account_request_generator::{create_account_delete_request, create_account_modify_request, create_login_request, create_logout_request, create_register_request};
use crate::request_generator::battle_ready_account_hash_request_generator::create_battle_ready_account_hash_request;
use crate::request_generator::battle_wait_queue_request_generator::create_battle_wait_queue_request;
use crate::request_generator::check_battle_prepare_request_generator::create_check_battle_prepare_request;
use crate::request_generator::client_program_request_generator::create_client_program_exit_request;
use crate::request_generator::account_deck_card_request_generator::{create_account_deck_card_list_request_form, create_account_deck_card_modify_request_form, create_account_deck_configuration_request_form};
use crate::request_generator::attach_general_energy_card_request_form_generator::create_attach_general_energy_card_request_form;
use crate::request_generator::game_deck_card_list_request_generator::create_game_deck_card_list_request;
use crate::request_generator::mulligan_request_generator::{create_check_opponent_mulligan_status_request_form, create_mulligan_request_form};
use crate::request_generator::session_request_generator::create_session_login_request;
use crate::request_generator::shop_request_generator::{create_data_to_display_in_shop_request, create_event_distribute_cards_request_form, create_execute_free_gacha_request_form, create_execute_shop_gacha_request_form};
use crate::request_generator::deploy_unit_request_form_generator::create_deploy_unit_request_form;
use crate::request_generator::energy_boost_support_request_form_generator::create_energy_boost_support_request_form;
use crate::game_turn::controller::game_turn_controller::GameTurnController;
use crate::game_winner_check::service::game_winner_check_service::GameWinnerCheckService;
use crate::game_winner_check::service::game_winner_check_service_impl::GameWinnerCheckServiceImpl;
use crate::mulligan::controller::mulligan_controller::MulliganController;
use crate::mulligan::controller::mulligan_controller_impl::MulliganControllerImpl;
use crate::request_generator::attach_field_energy_to_field_unit_request_form_generator::create_attach_field_energy_to_field_unit_request_form;
use crate::request_generator::attach_special_energy_card_request_form_generator::create_attach_special_energy_card_request_form;
use crate::request_generator::attack_unit_request_form_generator::create_attack_unit_request_form;
use crate::request_generator::attack_game_main_character_request_form_generator::create_attack_game_main_character_request_form;
use crate::request_generator::battle_finish_generator::create_battle_finish_request;
use crate::request_generator::battle_match_cancel_request_generator::create_battle_match_cancel_request;
use crate::request_generator::battle_start_request_form_generator::create_battle_start_request_form;
use crate::request_generator::check_connecting_request_generator::create_check_connecting_request;
use crate::request_generator::check_rockpaperscissors_winner_request_generator::create_check_rockpaperscissors_winner_request_form;
use crate::request_generator::deploy_non_targeting_attack_passive_skill_request_generator::create_deploy_non_targeting_attack_passive_skill_request_form;
use crate::request_generator::deploy_targeting_attack_passive_skill_request_generator::create_deploy_targeting_attack_passive_skill_request_form;
use crate::request_generator::deploy_targeting_attack_to_game_main_character_request_generator::create_deploy_targeting_attack_to_game_main_character_request_form;
use crate::request_generator::fake_battle_room_create_request_form_generator::create_fake_battle_room_create_request_form;
use crate::request_generator::game_card_item_request_form_generator::{create_add_field_energy_by_field_unit_health_point_item_request_form, create_catastrophic_damage_item_request_form, create_multiple_target_damage_by_field_unit_sacrifice_item_request_form, create_opponent_field_unit_energy_removal_item_request_form, create_target_death_item_request_form};
use crate::request_generator::game_next_turn_request_generator::create_game_turn_request_form;
use crate::request_generator::general_draw_support_request_form_generator::create_general_draw_support_request_form;
use crate::request_generator::non_targeting_active_skill_request_form_generator::create_non_targeting_active_skill_request_form;
use crate::request_generator::opponent_field_energy_remove_item_request_form_generator::create_opponent_field_energy_remove_item_request_form;
use crate::request_generator::remain_deck_card_count_request_generator::create_remain_deck_card_count_request;
use crate::request_generator::rockpaperscissors_request_generator::create_rockpaperscissors_request_form;
use crate::request_generator::search_unit_support_request_form_generator::{create_check_search_unit_support_available_request_form, create_search_unit_support_request_form};
use crate::request_generator::targeting_active_skill_request_form_generator::create_targeting_active_skill_request_form;
use crate::request_generator::what_is_the_room_number_request_generator::create_what_is_the_room_number_request;
use crate::request_generator::surrender_request_generator::create_surrender_request;
use crate::request_generator::fake_create_game_deck_card_list_request_generator::fake_create_game_deck_card_list_request;
use crate::request_generator::fake_get_all_specific_kind_cards_from_deck_request_form_generator::create_fake_get_all_specific_kind_cards_from_deck_request_form;
use crate::request_generator::fake_get_nether_from_deck_request_form_generator::create_fake_get_nether_from_deck_request_form;
use crate::request_generator::fake_multi_draw_request_form_generator::create_fake_multi_draw_request_form;
use crate::request_generator::targeting_attack_active_skill_to_game_main_character_request_generator::create_targeting_attack_active_skill_to_game_main_character_request_form;
use crate::request_generator::turn_start_non_targeting_attack_passive_skill_request_generator::create_turn_start_non_targeting_attack_passive_skill_request_form;
use crate::request_generator::turn_start_targeting_attack_passive_skill_request_generator::create_turn_start_targeting_attack_passive_skill_request_form;
use crate::request_generator::turn_start_targeting_attack_to_game_main_character_request_generator::create_turn_start_targeting_attack_to_game_main_character_request_form;
use crate::response_generator::response_type::ResponseType;
use crate::rock_paper_scissors::controller::rock_paper_scissors_controller::RockPaperScissorsController;
use crate::rock_paper_scissors::controller::rock_paper_scissors_controller_impl::RockPaperScissorsControllerImpl;
use crate::shop::controller::shop_controller::ShopController;
use crate::shop::controller::shop_controller_impl::ShopControllerImpl;
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
                if let Some(request) = create_battle_match_cancel_request(&data) {
                    println!("request generator: battle match request protocol");
                    let battle_wait_queue_service_mutex = BattleWaitQueueServiceImpl::get_instance();
                    let mut battle_wait_queue_service = battle_wait_queue_service_mutex.lock().await;

                    let response = battle_wait_queue_service.dequeue_player_id_from_wait_queue(request).await;
                    let response_type = Some(ResponseType::BATTLE_MATCH_CANCEL(response));
                    println!("response_type: {:?}", response_type);

                    response_type
                } else {
                    None
                }

            },
            14 => {
                // Check Battle Prepare (CHECK_BATTLE_PREPARE)
                if let Some(request) = create_check_battle_prepare_request(&data) {
                    let battle_ready_account_hash_service_mutex =
                        BattleReadyAccountHashServiceImpl::get_instance();

                    let mut battle_ready_account_hash_service_guard =
                        battle_ready_account_hash_service_mutex.lock().await;

                    let response =
                        battle_ready_account_hash_service_guard.check_prepare_for_battle(request).await;

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
            17 => {
                // Game Deck Card List
                if let Some(request) = create_game_deck_card_list_request(&data) {
                    let game_deck_card_service_mutex = GameDeckServiceImpl::get_instance();
                    let mut game_deck_card_service = game_deck_card_service_mutex.lock().await;

                    let response = game_deck_card_service.create_and_shuffle_deck(request).await;
                    let response_type = Some(ResponseType::BATTLE_START_SHUFFLED_GAME_DECK_CARD_LIST(response));

                    response_type
                } else {
                    None
                }
            },
            18 => {
                // Mulligan
                if let Some(request_form) = create_mulligan_request_form(&data) {
                    let mulligan_controller_mutex = MulliganControllerImpl::get_instance();
                    let mulligan_controller = mulligan_controller_mutex.lock().await;

                    let response_form = mulligan_controller.execute_mulligan_procedure(request_form).await;
                    let response_type = Some(ResponseType::CHANGE_FIRST_HAND(response_form));

                    response_type
                } else {
                    None
                }
            },
            19 => {
                // First Turn wait queue 최신 버전
                if let Some(request_form) = create_rockpaperscissors_request_form(&data) {
                    let rock_paper_scissors_controller_mutex = RockPaperScissorsControllerImpl::get_instance();
                    let mut rock_paper_scissors_controller_mutex_guard = rock_paper_scissors_controller_mutex.lock().await;

                    let response_form = rock_paper_scissors_controller_mutex_guard.execute_rock_paper_scissors_procedure(request_form).await;
                    let response_type = Some(ResponseType::ROCKPAPERSCISSORS(response_form));

                    response_type
                } else {
                    None
                }
            },
            20 => {
                // First Turn Decision 최신 버전
                if let Some(request_form) = create_check_rockpaperscissors_winner_request_form(&data) {
                    let rock_paper_scissors_controller_mutex = RockPaperScissorsControllerImpl::get_instance();
                    let mut rock_paper_scissors_controller_mutex_guard = rock_paper_scissors_controller_mutex.lock().await;

                    let response_form = rock_paper_scissors_controller_mutex_guard.execute_check_rock_paper_scissors_winner_procedure(request_form).await;
                    let response_type = Some(ResponseType::CHECK_ROCKPAPERSCISSORS_WINNER(response_form));

                    response_type
                } else {
                    None
                }
            },
            21 => {
                // Battle Start
                if let Some(request_form) = create_battle_start_request_form(&data) {
                    let battle_start_controller_mutex = BattleStartControllerImpl::get_instance();
                    let mut battle_start_controller_mutex_guard = battle_start_controller_mutex.lock().await;

                    let response_form = battle_start_controller_mutex_guard.request_to_start_battle(request_form).await;
                    let response_type = Some(ResponseType::BATTLE_START(response_form));

                    response_type
                } else {
                    None
                }
            },
            22 => {
                // Mulligan
                if let Some(request_form) = create_check_opponent_mulligan_status_request_form(&data) {
                    let mulligan_controller_mutex = MulliganControllerImpl::get_instance();
                    let mulligan_controller = mulligan_controller_mutex.lock().await;

                    let response_form = mulligan_controller.check_opponent_mulligan_status(request_form).await;
                    let response_type = Some(ResponseType::CHECK_OPPONENT_MULLIGAN(response_form));

                    response_type
                } else {
                    None
                }
            },
            31 => {
                // Account Card List
                if let Some(request) = create_account_card_list_request(&data) {
                    let account_card_service_mutex = AccountCardServiceImpl::get_instance();
                    let mut account_card_service_mutex_guard = account_card_service_mutex.lock().await;

                    let response = account_card_service_mutex_guard.account_card_list(request).await;
                    let response_type = Some(ResponseType::ACCOUNT_CARD_LIST(response));

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
                // Account Deck (Name) Modify
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
            44 => {
                // Account Deck (All info) Delete
                if let Some(request) = create_deck_delete_request(&data) {
                    let account_deck_service_mutex = AccountDeckServiceImpl::get_instance();
                    let mut account_deck_service = account_deck_service_mutex.lock().await;

                    let response = account_deck_service.account_deck_delete(request).await;
                    let response_type = Some(ResponseType::ACCOUNT_DECK_DELETE(response));

                    response_type
                } else {
                    None
                }
            },
            51 => {
                // Account Deck Card Configuration
                if let Some(request_form) = create_account_deck_configuration_request_form(&data) {
                    let deck_card_controller_mutex = AccountDeckCardControllerImpl::get_instance();
                    let mut deck_card_controller_mutex_guard = deck_card_controller_mutex.lock().await;

                    let response_form = deck_card_controller_mutex_guard.deck_configuration_register(request_form).await;
                    let response_type = Some(ResponseType::DECK_CARD_CONFIGURATION(response_form));

                    response_type
                } else {
                    None
                }
            },
            52 => {
                // Account Deck Card List
                if let Some(request_form) = create_account_deck_card_list_request_form(&data) {
                    let deck_card_controller_mutex = AccountDeckCardControllerImpl::get_instance();
                    let mut deck_card_controller_mutex_guard = deck_card_controller_mutex.lock().await;

                    let response_form = deck_card_controller_mutex_guard.deck_card_list(request_form).await;
                    let response_type = Some(ResponseType::DECK_CARD_LIST(response_form));

                    response_type
                } else {
                    None
                }
            },
            53 => {
                // Account Deck Card Modify
                if let Some(request_form) = create_account_deck_card_modify_request_form(&data) {
                    let deck_card_controller_mutex = AccountDeckCardControllerImpl::get_instance();
                    let mut deck_card_controller_mutex_guard = deck_card_controller_mutex.lock().await;

                    let response_form = deck_card_controller_mutex_guard.deck_card_modify(request_form).await;
                    let response_type = Some(ResponseType::DECK_CARD_MODIFY(response_form));

                    response_type
                } else {
                    None
                }
            },
            71 => {
                // Shop Data Display
                if let Some(request) = create_data_to_display_in_shop_request(&data) {
                    let shop_service_mutex = ShopServiceImpl::get_instance();
                    let mut shop_service = shop_service_mutex.lock().await;

                    let response = shop_service.data_to_display_in_shop(request).await;
                    let response_type = Some(ResponseType::SHOP_DATA(response));

                    response_type
                } else {
                    None
                }
            },
            72 => {
                // Shop Get Specific Race Card Default
                if let Some(request) = create_execute_shop_gacha_request_form(&data) {
                    let shop_controller_mutex = ShopControllerImpl::get_instance();
                    let mut shop_controller = shop_controller_mutex.lock().await;

                    let response = shop_controller.execute_shop_gacha(request).await;
                    let response_type = Some(ResponseType::SHOP_GACHA(response));

                    response_type
                } else {
                    None
                }
            },
            73 => {
                // Shop Free Get Specific Race Card
                if let Some(request) = create_execute_free_gacha_request_form(&data) {
                    let shop_controller_mutex = ShopControllerImpl::get_instance();
                    let mut shop_controller = shop_controller_mutex.lock().await;

                    let response = shop_controller.execute_free_gacha(request).await;
                    let response_type = Some(ResponseType::FREE_GACHA(response));

                    response_type
                } else {
                    None
                }
            },
            90 => {
                // Shop Distirbute Cards Deck
                if let Some(request) = create_event_distribute_cards_request_form(&data) {
                    let shop_controller_mutex = ShopControllerImpl::get_instance();
                    let mut shop_controller = shop_controller_mutex.lock().await;

                    let response = shop_controller.event_distribute_cards(request).await;
                    let response_type = Some(ResponseType::EVENT_DISTRIBUTE_CARDS(response));

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
            101=> {
                // Battlefield info remain my deck card count
                if let Some(request) = create_remain_deck_card_count_request(&data) {
                let battle_field_info_service_mutex = BattleFieldInfoServiceImpl::get_instance();
                let mut battle_field_info_service = battle_field_info_service_mutex.lock().await;

                let response = battle_field_info_service.get_remain_deck_card_count(request).await;
                let response_type = Some(ResponseType::REMAIN_DECK_CARD_COUNT(response));

                    response_type
                }
                else
                {
                    None
                }
            },
            501 => {
                // Check Search Unit Support Usage
                if let Some(request_form) = create_check_search_unit_support_available_request_form(&data) {
                    let game_card_support_controller_mutex = GameCardSupportControllerImpl::get_instance();
                    let game_card_support_controller = game_card_support_controller_mutex.lock().await;

                    let response_form = game_card_support_controller.check_search_unit_support_available(request_form).await;
                    let response_type = Some(ResponseType::CHECK_SEARCH_UNIT_SUPPORT_USAGE(response_form));

                    response_type
                } else {
                    None
                }
            },
            1000 => {
                // Unit attack
                if let Some(request_form) = create_attack_unit_request_form(&data) {
                    let game_card_unit_controller_mutex = GameCardUnitControllerImpl::get_instance();
                    let game_card_unit_controller = game_card_unit_controller_mutex.lock().await;

                    let response_form = game_card_unit_controller.request_to_attack_unit(request_form).await;
                    let response_type = Some(ResponseType::ATTACK_UNIT(response_form));

                    response_type
                } else {
                    None
                }
            },
            1001 => {
                // Unit use targeting active skill
                if let Some(request_form) = create_targeting_active_skill_request_form(&data) {
                    let game_card_active_skill_controller_mutex = GameCardActiveSkillControllerImpl::get_instance();
                    let game_card_active_skill_controller = game_card_active_skill_controller_mutex.lock().await;

                    let response_form = game_card_active_skill_controller.request_targeting_active_skill(request_form).await;
                    let response_type = Some(ResponseType::TARGETING_ACTIVE_SKILL(response_form));

                    response_type
                } else {
                    None
                }
            },
            1002 => {
                // Unit use non-targeting active skill
                if let Some(request_form) = create_non_targeting_active_skill_request_form(&data) {
                    let game_card_active_skill_controller_mutex = GameCardActiveSkillControllerImpl::get_instance();
                    let game_card_active_skill_controller = game_card_active_skill_controller_mutex.lock().await;

                    let response_form = game_card_active_skill_controller.request_non_targeting_active_skill(request_form).await;
                    let response_type = Some(ResponseType::NON_TARGETING_ACTIVE_SKILL(response_form));

                    response_type
                } else {
                    None
                }
            },
            1003 => {
                // Attach field energy to field unit
                if let Some(request_form) = create_attach_field_energy_to_field_unit_request_form(&data) {
                    let game_field_energy_controller_mutex = GameFieldEnergyControllerImpl::get_instance();
                    let game_field_energy_controller = game_field_energy_controller_mutex.lock().await;

                    let response_form = game_field_energy_controller.request_to_attach_field_energy_to_field_unit(request_form).await;
                    let response_type = Some(ResponseType::ATTACH_FIELD_ENERGY_TO_UNIT(response_form));

                    response_type
                } else {
                    None
                }
            },
            1004 => {
                // Unit Card Usage
                if let Some(request_form) = create_deploy_unit_request_form(&data) {
                    let game_card_unit_controller_mutex = GameCardUnitControllerImpl::get_instance();
                    let game_card_unit_controller = game_card_unit_controller_mutex.lock().await;

                    let response_form = game_card_unit_controller.request_to_deploy_unit(request_form).await;
                    let response_type = Some(ResponseType::DEPLOY_UNIT_USAGE(response_form));

                    response_type
                } else {
                    None
                }
            },
            1005 => {
                // Energy Boost Support Usage
                if let Some(request_form) = create_energy_boost_support_request_form(&data) {
                    let game_card_support_controller_mutex = GameCardSupportControllerImpl::get_instance();
                    let game_card_support_controller = game_card_support_controller_mutex.lock().await;

                    let response_form = game_card_support_controller.request_to_use_energy_boost_support(request_form).await;
                    let response_type = Some(ResponseType::ENERGY_BOOST_SUPPORT_USAGE(response_form));

                    response_type
                } else {
                    None
                }
            },
            1006 => {
                // Target Death Item Card Usage
                if let Some(request_form) = create_target_death_item_request_form(&data) {
                    let game_card_item_controller_mutex = GameCardItemControllerImpl::get_instance();
                    let game_card_item_controller = game_card_item_controller_mutex.lock().await;

                    let response_form = game_card_item_controller.request_to_use_target_death_item(request_form).await;
                    let response_type = Some(ResponseType::TARGET_DEATH_ITEM_USAGE(response_form));

                    response_type
                } else {
                    None
                }
            },
            1007 => {
                // Catastrophic Damage Item Card Usage
                if let Some(request_form) = create_catastrophic_damage_item_request_form(&data) {
                    let game_card_item_controller_mutex = GameCardItemControllerImpl::get_instance();
                    let game_card_item_controller = game_card_item_controller_mutex.lock().await;

                    let response_form = game_card_item_controller.request_to_use_catastrophic_damage_item(request_form).await;
                    let response_type = Some(ResponseType::CATASTROPHIC_DAMAGE_ITEM_USAGE(response_form));

                    response_type
                } else {
                    None
                }
            },
            1008 => {
                // Remove Opponent Field Energy Support->Item Usage
                if let Some(request_form) = create_opponent_field_energy_remove_item_request_form(&data) {
                    let game_card_item_controller_mutex = GameCardItemControllerImpl::get_instance();
                    let game_card_item_controller = game_card_item_controller_mutex.lock().await;

                    let response_form = game_card_item_controller.request_to_use_remove_opponent_field_energy_item(request_form).await;
                    let response_type = Some(ResponseType::REMOVE_OPPONENT_FIELD_ENERGY_ITEM_USAGE(response_form));
                    response_type
                } else {
                    None
                }
            },
            1009 => {
                // Add Field Energy by Field Unit HP Item Card Usage
                if let Some(request_form) = create_add_field_energy_by_field_unit_health_point_item_request_form(&data) {
                    let game_card_item_controller_mutex = GameCardItemControllerImpl::get_instance();
                    let game_card_item_controller = game_card_item_controller_mutex.lock().await;

                    let response_form = game_card_item_controller.request_to_use_add_field_energy_with_field_unit_health_point(request_form).await;
                    let response_type = Some(ResponseType::ADD_FIELD_ENERGY_BY_FIELD_UNIT_HEALTH_POINT_ITEM_USAGE(response_form));

                    response_type
                } else {
                    None
                }
            },
            1010 => {
                // Energy Card Usage
                if let Some(request_form) = create_attach_general_energy_card_request_form(&data) {
                    let game_card_energy_controller_mutex = GameCardEnergyControllerImpl::get_instance();
                    let game_card_energy_controller = game_card_energy_controller_mutex.lock().await;

                    let response_form = game_card_energy_controller.request_to_attach_general_energy(request_form).await;
                    let response_type = Some(ResponseType::ATTACH_GENERAL_ENERGY(response_form));

                    response_type
                } else {
                    None
                }
            },
            1011 => {
                // Search Unit Support Usage
                if let Some(request_form) = create_search_unit_support_request_form(&data) {
                    let game_card_support_controller_mutex = GameCardSupportControllerImpl::get_instance();
                    let game_card_support_controller = game_card_support_controller_mutex.lock().await;

                    let response_form = game_card_support_controller.request_to_use_search_unit_support(request_form).await;
                    let response_type = Some(ResponseType::SEARCH_UNIT_SUPPORT_USAGE(response_form));

                    response_type
                } else {
                    None
                }
            },
            1012 => {
                // Special Energy Card Usage
                if let Some(request_form) = create_attach_special_energy_card_request_form(&data) {
                    let game_card_energy_controller_mutex = GameCardEnergyControllerImpl::get_instance();
                    let game_card_energy_controller = game_card_energy_controller_mutex.lock().await;

                    let response_form = game_card_energy_controller.request_to_attach_special_energy(request_form).await;
                    let response_type = Some(ResponseType::ATTACH_SPECIAL_ENERGY(response_form));

                    response_type
                } else {
                    None
                }
            },
            1013 => {
                // Draw Support Usage
                if let Some(request_form) = create_general_draw_support_request_form(&data) {
                    let game_card_support_controller_mutex = GameCardSupportControllerImpl::get_instance();
                    let game_card_support_controller = game_card_support_controller_mutex.lock().await;

                    let response_form = game_card_support_controller.request_to_use_draw_support(request_form).await;
                    let response_type = Some(ResponseType::GENERAL_DRAW_SUPPORT_USAGE(response_form));

                    response_type
                } else {
                    None
                }
            },
            1014 => {
                // Multiple Target Damage by Field Unit Sacrifice Item Card Usage
                if let Some(request_form) = create_multiple_target_damage_by_field_unit_sacrifice_item_request_form(&data) {
                    let game_card_item_controller_mutex = GameCardItemControllerImpl::get_instance();
                    let game_card_item_controller = game_card_item_controller_mutex.lock().await;

                    let response_form = game_card_item_controller.request_to_use_applying_multiple_target_damage_by_field_unit_death_item(request_form).await;
                    let response_type = Some(ResponseType::MULTIPLE_TARGET_DAMAGE_BY_FIELD_UNIT_SACRIFICE_ITEM_USAGE(response_form));

                    response_type
                } else {
                    None
                }
            },
            1015 => {
                // Opponent Field Unit Energy Removal Item Usage
                if let Some(request_form) = create_opponent_field_unit_energy_removal_item_request_form(&data) {
                    let game_card_item_controller_mutex = GameCardItemControllerImpl::get_instance();
                    let game_card_item_controller = game_card_item_controller_mutex.lock().await;

                    let response_form = game_card_item_controller.request_to_use_opponent_field_unit_energy_removal_item(request_form).await;
                    let response_type = Some(ResponseType::OPPONENT_FIElD_UNIT_ENERGY_REMOVAL_ITEM_USAGE(response_form));

                    response_type
                } else {
                    None
                }
            },
            1016 => {
                // Attack Main Character
                if let Some(request_form) = create_attack_game_main_character_request_form(&data) {
                    let game_card_unit_controller_mutex = GameCardUnitControllerImpl::get_instance();
                    let game_card_unit_controller = game_card_unit_controller_mutex.lock().await;

                    let response_form = game_card_unit_controller.request_to_attack_game_main_character(request_form).await;
                    let response_type = Some(ResponseType::ATTACK_MAIN_CHARACTER(response_form));

                    response_type
                } else {
                    None
                }
            },
            1017 => {
                // Active Skill Targeting Attack To Game Main Character
                if let Some(request_form) = create_targeting_attack_active_skill_to_game_main_character_request_form(&data) {
                    let game_card_active_skill_controller_mutex = GameCardActiveSkillControllerImpl::get_instance();
                    let game_card_active_skill_controller = game_card_active_skill_controller_mutex.lock().await;

                    let response_form = game_card_active_skill_controller.request_targeting_active_skill_to_game_main_character(request_form).await;
                    let response_type = Some(ResponseType::TARGETING_ATTACK_ACTIVE_SKILL_TO_GAME_MAIN_CHARACTER(response_form));

                    response_type
                } else {
                    None
                }

            },
            2000 => {
                // Deploy Passive Targeting Attack Unit
                if let Some(request_form) = create_deploy_targeting_attack_passive_skill_request_form(&data) {
                    let game_card_passive_skill_controller_mutex = GameCardPassiveSkillControllerImpl::get_instance();
                    let game_card_passive_skill_controller = game_card_passive_skill_controller_mutex.lock().await;

                    let response_form = game_card_passive_skill_controller.request_deploy_targeting_attack_passive_skill(request_form).await;
                    let response_type = Some(ResponseType::DEPLOY_TARGETING_ATTACK_PASSIVE_SKILL(response_form));

                    response_type
                } else {
                    None
                }
            },
            2001 => {
                // Deploy Passive Targeting Attack To Game Main Character
                if let Some(request_form) = create_deploy_targeting_attack_to_game_main_character_request_form(&data) {
                    let game_card_passive_skill_controller_mutex = GameCardPassiveSkillControllerImpl::get_instance();
                    let game_card_passive_skill_controller = game_card_passive_skill_controller_mutex.lock().await;

                    let response_form = game_card_passive_skill_controller.request_deploy_targeting_attack_to_game_main_character(request_form).await;
                    let response_type = Some(ResponseType::DEPLOY_TARGETING_ATTACK_TO_MAIN_CHARACTER(response_form));

                    response_type
                } else {
                    None
                }
            },
            2002 => {
                // Deploy Passive Non Targeting Attack
                if let Some(request_form) = create_deploy_non_targeting_attack_passive_skill_request_form(&data) {
                    let game_card_passive_skill_controller_mutex = GameCardPassiveSkillControllerImpl::get_instance();
                    let game_card_passive_skill_controller = game_card_passive_skill_controller_mutex.lock().await;

                    let response_form = game_card_passive_skill_controller.request_deploy_non_targeting_attack_passive_skill(request_form).await;
                    let response_type = Some(ResponseType::DEPLOY_NON_TARGETING_ATTACK_PASSIVE_SKILL(response_form));

                    response_type
                } else {
                    None
                }
            },
            2010 => {
                // Turn Start Passive Targeting Attack Unit
                if let Some(request_form) = create_turn_start_targeting_attack_passive_skill_request_form(&data) {
                    let game_card_passive_skill_controller_mutex = GameCardPassiveSkillControllerImpl::get_instance();
                    let game_card_passive_skill_controller = game_card_passive_skill_controller_mutex.lock().await;

                    let response_form = game_card_passive_skill_controller.request_turn_start_targeting_attack_passive_skill(request_form).await;
                    let response_type = Some(ResponseType::TURN_START_TARGETING_ATTACK_PASSIVE_SKILL(response_form));

                    response_type
                } else {
                    None
                }
            },
            2011 => {
                // Turn Start Passive Targeting Attack To Game Main Character
                if let Some(request_form) = create_turn_start_targeting_attack_to_game_main_character_request_form(&data) {
                    let game_card_passive_skill_controller_mutex = GameCardPassiveSkillControllerImpl::get_instance();
                    let game_card_passive_skill_controller = game_card_passive_skill_controller_mutex.lock().await;

                    let response_form = game_card_passive_skill_controller.request_turn_start_targeting_attack_to_game_main_character(request_form).await;
                    let response_type = Some(ResponseType::TURN_START_TARGETING_ATTACK_TO_MAIN_CHARACTER(response_form));

                    response_type
                } else {
                    None
                }
            },
            2012 => {
                // Turn Start Passive Non Targeting Attack
                if let Some(request_form) = create_turn_start_non_targeting_attack_passive_skill_request_form(&data) {
                    let game_card_passive_skill_controller_mutex = GameCardPassiveSkillControllerImpl::get_instance();
                    let game_card_passive_skill_controller = game_card_passive_skill_controller_mutex.lock().await;

                    let response_form = game_card_passive_skill_controller.request_turn_start_non_targeting_attack_passive_skill(request_form).await;
                    let response_type = Some(ResponseType::TURN_START_NON_TARGETING_ATTACK_PASSIVE_SKILL(response_form));

                    response_type
                } else {
                    None
                }
            },
            3333 => {
                // Game Next Turn
                if let Some(request) = create_game_turn_request_form(&data) {
                    let game_turn_controller_impl_mutex = GameTurnControllerImpl::get_instance();
                    let mut game_turn_controller = game_turn_controller_impl_mutex.lock().await;

                    let response = game_turn_controller.request_turn_end(request).await;
                    let response_type = Some(ResponseType::GAME_NEXT_TURN(response));

                    response_type
                } else {
                    None
                }
            },
            4441 => {
                // Check Connecting
                if let Some(request) = create_check_connecting_request(&data) {
                    let check_connecting_service_mutex = CheckConnectingServiceImpl::get_instance();
                    let mut check_connecting_service = check_connecting_service_mutex.lock().await;

                    let response = check_connecting_service.checked_response(request).await;
                    let response_type = Some(ResponseType::CHECK_CONNECTING(response));

                    response_type
                } else {
                    None
                }
            },
            4442 => {
                // Battle finish
                if let Some(request) = create_battle_finish_request(&data) {
                    let battle_finish_service_mutex = BattleFinishServiceImpl::get_instance();
                    let mut battle_finish_service = battle_finish_service_mutex.lock().await;

                    let response = battle_finish_service.battle_finish_for_player_battle(request).await;
                    let response_type = Some(ResponseType::BATTLE_FINISH(response));

                    response_type
                } else {
                    None
                }
            },
            4443 => {
                // Game Surrender
                if let Some(request) = create_surrender_request(&data) {
                    let game_winner_check_service_mutex = GameWinnerCheckServiceImpl::get_instance();
                    let mut game_winner_check_service = game_winner_check_service_mutex.lock().await;

                    let response = game_winner_check_service.set_game_winner_by_surrender(request).await;
                    let response_type = Some(ResponseType::GAME_SURRENDER(response));

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
            8001 => {
                // Fake Battle Room Test
                if let Some(request) = create_fake_battle_room_create_request_form(&data) {
                    let fake_battle_room_controller_mutex = FakeBattleRoomControllerImpl::get_instance();
                    let mut fake_battle_room_controller = fake_battle_room_controller_mutex.lock().await;

                    let response = fake_battle_room_controller.request_to_create_fake_battle_room(request).await;
                    let response_type = Some(ResponseType::FAKE_BATTLE_ROOM_CREATION(response));

                    response_type
                } else {
                    None
                }
            },
            8002 => {
                // TODO: Mocking 전용이라 8000번대에 있으면 안됨
                //드로우 30장 하고 시작 테스트용 Game Deck Card List
                if let Some(request) = fake_create_game_deck_card_list_request(&data) {
                    let game_deck_card_service_mutex = GameDeckServiceImpl::get_instance();
                    let mut game_deck_card_service = game_deck_card_service_mutex.lock().await;

                    let response = game_deck_card_service.fake_create_and_shuffle_deck(request).await;
                    let response_type = Some(ResponseType::BATTLE_START_SHUFFLED_GAME_DECK_CARD_LIST(response));

                    response_type
                } else {
                    None
                }
            },
            8003 => {
                if let Some(request) = create_fake_multi_draw_request_form(&data) {
                    let fake_battle_room_controller_mutex = FakeBattleRoomControllerImpl::get_instance();
                    let mut fake_battle_room_controller = fake_battle_room_controller_mutex.lock().await;

                    let response = fake_battle_room_controller.request_to_fake_multi_draw(request).await;
                    let response_type = Some(ResponseType::FAKE_MULTI_DRAW(response));

                    response_type
                } else {
                    None
                }
            },
            8004 => {
                if let Some(request) = create_fake_get_nether_from_deck_request_form(&data) {
                    let fake_battle_room_controller_mutex = FakeBattleRoomControllerImpl::get_instance();
                    let mut fake_battle_room_controller = fake_battle_room_controller_mutex.lock().await;

                    let response = fake_battle_room_controller.request_to_get_nether_from_deck(request).await;
                    let response_type = Some(ResponseType::FAKE_GET_NETHER(response));

                    response_type
                } else {
                    None
                }
            },
            8005 => {
                if let Some(request) = create_fake_get_all_specific_kind_cards_from_deck_request_form(&data) {
                    let fake_battle_room_controller_mutex = FakeBattleRoomControllerImpl::get_instance();
                    let mut fake_battle_room_controller = fake_battle_room_controller_mutex.lock().await;

                    let response = fake_battle_room_controller.request_to_get_all_cards_of_specific_kind_from_deck(request).await;
                    let response_type = Some(ResponseType::FAKE_GET_SPECIFIC_KIND_DECK_CARD(response));

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