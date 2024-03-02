use std::sync::Arc;
use async_trait::async_trait;
use lazy_static::lazy_static;

use tokio::sync::Mutex as AsyncMutex;
use crate::battle_room::service::battle_room_service::BattleRoomService;
use crate::battle_room::service::battle_room_service_impl::BattleRoomServiceImpl;

use crate::game_card_energy::controller::game_card_energy_controller::GameCardEnergyController;
use crate::game_card_energy::controller::request_form::attach_general_energy_card_request_form::AttachGeneralEnergyCardRequestForm;
use crate::game_card_energy::controller::request_form::attach_special_energy_card_request_form::AttachSpecialEnergyCardRequestForm;
use crate::game_card_energy::controller::response_form::attach_general_energy_card_response_form::AttachGeneralEnergyCardResponseForm;
use crate::game_card_energy::controller::response_form::attach_special_energy_card_response_form::AttachSpecialEnergyCardResponseForm;
use crate::game_card_energy::service::game_card_energy_service::GameCardEnergyService;
use crate::game_card_energy::service::game_card_energy_service_impl::GameCardEnergyServiceImpl;
use crate::game_field_unit::service::game_field_unit_service::GameFieldUnitService;
use crate::game_field_unit::service::game_field_unit_service_impl::GameFieldUnitServiceImpl;
use crate::game_hand::service::game_hand_service::GameHandService;
use crate::game_hand::service::game_hand_service_impl::GameHandServiceImpl;
use crate::game_protocol_validation::service::game_protocol_validation_service::GameProtocolValidationService;
use crate::game_protocol_validation::service::game_protocol_validation_service_impl::GameProtocolValidationServiceImpl;
use crate::game_tomb::service::game_tomb_service::GameTombService;
use crate::game_tomb::service::game_tomb_service_impl::GameTombServiceImpl;
use crate::notify_player_action::service::notify_player_action_service::NotifyPlayerActionService;
use crate::notify_player_action::service::notify_player_action_service_impl::NotifyPlayerActionServiceImpl;
use crate::notify_player_action_info::service::notify_player_action_info_service::NotifyPlayerActionInfoService;
use crate::notify_player_action_info::service::notify_player_action_info_service_impl::NotifyPlayerActionInfoServiceImpl;
use crate::redis::service::redis_in_memory_service::RedisInMemoryService;
use crate::redis::service::redis_in_memory_service_impl::RedisInMemoryServiceImpl;
use crate::redis::service::request::get_value_with_key_request::GetValueWithKeyRequest;
use crate::ui_data_generator::service::ui_data_generator_service::UiDataGeneratorService;
use crate::ui_data_generator::service::ui_data_generator_service_impl::UiDataGeneratorServiceImpl;

pub struct GameCardEnergyControllerImpl {
    game_hand_service: Arc<AsyncMutex<GameHandServiceImpl>>,
    game_tomb_service: Arc<AsyncMutex<GameTombServiceImpl>>,
    battle_room_service: Arc<AsyncMutex<BattleRoomServiceImpl>>,
    game_field_unit_service: Arc<AsyncMutex<GameFieldUnitServiceImpl>>,
    redis_in_memory_service: Arc<AsyncMutex<RedisInMemoryServiceImpl>>,
    game_card_energy_service: Arc<AsyncMutex<GameCardEnergyServiceImpl>>,
    notify_player_action_service: Arc<AsyncMutex<NotifyPlayerActionServiceImpl>>,
    game_protocol_validation_service: Arc<AsyncMutex<GameProtocolValidationServiceImpl>>,
    notify_player_action_info_service: Arc<AsyncMutex<NotifyPlayerActionInfoServiceImpl>>,
    ui_data_generator_service: Arc<AsyncMutex<UiDataGeneratorServiceImpl>>,
}

impl GameCardEnergyControllerImpl {
    pub fn new(game_hand_service: Arc<AsyncMutex<GameHandServiceImpl>>,
               game_tomb_service: Arc<AsyncMutex<GameTombServiceImpl>>,
               battle_room_service: Arc<AsyncMutex<BattleRoomServiceImpl>>,
               game_field_unit_service: Arc<AsyncMutex<GameFieldUnitServiceImpl>>,
               redis_in_memory_service: Arc<AsyncMutex<RedisInMemoryServiceImpl>>,
               game_card_energy_service: Arc<AsyncMutex<GameCardEnergyServiceImpl>>,
               notify_player_action_service: Arc<AsyncMutex<NotifyPlayerActionServiceImpl>>,
               game_protocol_validation_service: Arc<AsyncMutex<GameProtocolValidationServiceImpl>>,
               notify_player_action_info_service: Arc<AsyncMutex<NotifyPlayerActionInfoServiceImpl>>,
               ui_data_generator_service: Arc<AsyncMutex<UiDataGeneratorServiceImpl>>,) -> Self {

        GameCardEnergyControllerImpl {
            game_hand_service,
            game_tomb_service,
            battle_room_service,
            game_field_unit_service,
            redis_in_memory_service,
            game_card_energy_service,
            notify_player_action_service,
            game_protocol_validation_service,
            notify_player_action_info_service,
            ui_data_generator_service
        }
    }
    pub fn get_instance() -> Arc<AsyncMutex<GameCardEnergyControllerImpl>> {
        lazy_static! {
            static ref INSTANCE: Arc<AsyncMutex<GameCardEnergyControllerImpl>> =
                Arc::new(
                    AsyncMutex::new(
                        GameCardEnergyControllerImpl::new(
                            GameHandServiceImpl::get_instance(),
                            GameTombServiceImpl::get_instance(),
                            BattleRoomServiceImpl::get_instance(),
                            GameFieldUnitServiceImpl::get_instance(),
                            RedisInMemoryServiceImpl::get_instance(),
                            GameCardEnergyServiceImpl::get_instance(),
                            NotifyPlayerActionServiceImpl::get_instance(),
                            GameProtocolValidationServiceImpl::get_instance(),
                            NotifyPlayerActionInfoServiceImpl::get_instance(),
                            UiDataGeneratorServiceImpl::get_instance())));
        }
        INSTANCE.clone()
    }

    async fn is_valid_session(&self, request: GetValueWithKeyRequest) -> i32 {
        let redis_in_memory_service_guard =
            self.redis_in_memory_service.lock().await;

        let session_validation_response =
            redis_in_memory_service_guard.get_value_with_key(request).await;

        let value_string = session_validation_response.get_value();

        value_string.parse::<i32>().unwrap_or_else(|_| { -1 })
    }
}

#[async_trait]
impl GameCardEnergyController for GameCardEnergyControllerImpl {
    async fn request_to_attach_general_energy(
        &self, attach_energy_request_form: AttachGeneralEnergyCardRequestForm)
        -> AttachGeneralEnergyCardResponseForm {

        println!("GameCardEnergyControllerImpl: request_to_attach_energy()");

        // 1. 세션 아이디를 검증합니다.
        let account_unique_id = self.is_valid_session(
            attach_energy_request_form.to_session_validation_request()).await;

        if account_unique_id == -1 {
            println!("유효하지 않은 세션입니다.");
            return AttachGeneralEnergyCardResponseForm::default()
        }

        let mut game_protocol_validation_service_guard =
            self.game_protocol_validation_service.lock().await;

        let is_this_your_turn_response =
            game_protocol_validation_service_guard.is_this_your_turn(
                attach_energy_request_form
                    .to_is_this_your_turn_request(account_unique_id)).await;

        if !is_this_your_turn_response.is_success() {
            println!("당신의 턴이 아닙니다.");
            return AttachGeneralEnergyCardResponseForm::default()
        }

        let energy_card_id_string = attach_energy_request_form.get_energy_card_id();
        let energy_card_id = energy_card_id_string.parse::<i32>().unwrap();

        // 2. GameProtocolValidation Service 호출하여 Hand 에 있는지 확인하여 해킹 여부 검증
        let check_protocol_hacking_response =
            game_protocol_validation_service_guard.check_protocol_hacking(
                attach_energy_request_form
                    .to_check_protocol_hacking_request(account_unique_id, energy_card_id)).await;

        if !check_protocol_hacking_response.is_success() {
            println!("해킹범을 검거합니다!");
            return AttachGeneralEnergyCardResponseForm::default()
        }

        // 3. CardKinds Service 를 호출하여 실제 에너지 카드가 맞는지 확인
        let is_it_energy_response =
            game_protocol_validation_service_guard.is_it_energy_card(
                attach_energy_request_form
                    .to_is_it_energy_card_request(energy_card_id)).await;

        if !is_it_energy_response.is_success() {
            println!("에너지 카드가 아닌데 요청이 왔으므로 당신도 해킹범입니다.");
            return AttachGeneralEnergyCardResponseForm::default()
        }

        drop(game_protocol_validation_service_guard);

        // 4. Hand Service 호출하여 에너지 카드 사용
        let mut game_hand_service_guard =
            self.game_hand_service.lock().await;

        let usage_hand_card_id =
            game_hand_service_guard.use_energy_card(
                attach_energy_request_form
                    .to_use_game_hand_energy_card_request(
                        account_unique_id,
                        energy_card_id)).await.get_found_energy_card_id();

        let mut game_tomb_service_guard =
            self.game_tomb_service.lock().await;

        game_tomb_service_guard.add_used_card_to_tomb(
            attach_energy_request_form
                .to_place_to_tomb_request(account_unique_id, usage_hand_card_id)).await;

        drop(game_tomb_service_guard);

        // 5. Energy 카드 Summarized Effect 산출
        let mut game_card_energy_service_guard =
            self.game_card_energy_service.lock().await;

        let energy_card_effect_response =
            game_card_energy_service_guard.summary_energy_effect(
                attach_energy_request_form
                    .to_summary_energy_card_effect_request(energy_card_id)).await;

        drop(game_card_energy_service_guard);

        let unit_card_index_string = attach_energy_request_form.get_unit_card_index();
        let unit_card_index = unit_card_index_string.parse::<i32>().unwrap();

        // 6. Battle Field 유닛에 에너지 붙이기
        let mut game_field_unit_service_guard =
            self.game_field_unit_service.lock().await;

        let attach_energy_to_field_unit_response =
            game_field_unit_service_guard.attach_energy_to_field_unit_index(
                attach_energy_request_form
                    .to_attach_single_energy_to_field_unit_request(
                        account_unique_id,
                        unit_card_index,
                        energy_card_effect_response.get_race())).await;

        if !attach_energy_to_field_unit_response.is_success() {
            println!("필드에 유닛에게 에너지를 부착하는 과정에서 문제가 발생하였습니다.");
            return AttachGeneralEnergyCardResponseForm::default()
        }

        let updated_energy_map_of_unit =
            game_field_unit_service_guard.get_current_attached_energy_of_field_unit_by_index(
                attach_energy_request_form
                    .to_get_current_attached_energy_of_unit_by_index_request(
                        account_unique_id,
                        unit_card_index)).await.get_current_attached_energy_map().clone();

        let mut ui_data_generator_service_guard =
            self.ui_data_generator_service.lock().await;

        let generate_use_my_hand_card_data_response =
            ui_data_generator_service_guard.generate_use_my_hand_card_data(
                attach_energy_request_form
                    .to_generate_use_my_hand_card_data_request(
                        usage_hand_card_id)).await;

        let generate_my_specific_unit_energy_data_response =
            ui_data_generator_service_guard.generate_my_specific_unit_energy_data(
                attach_energy_request_form
                    .to_generate_my_specific_unit_energy_data_request(
                        unit_card_index,
                        updated_energy_map_of_unit)).await;

        drop(ui_data_generator_service_guard);

        // 7. 상대방의 고유 id 값을 확보
        let battle_room_service_guard =
            self.battle_room_service.lock().await;

        let opponent_unique_id =
            battle_room_service_guard.find_opponent_by_account_unique_id(
                attach_energy_request_form
                    .to_find_opponent_by_account_id_request(
                        account_unique_id)).await.get_opponent_unique_id();

        drop(battle_room_service_guard);

        let mut notify_player_action_info_service_guard =
            self.notify_player_action_info_service.lock().await;

        let notice_response =
            notify_player_action_info_service_guard.notice_use_general_energy_card_to_my_specific_unit(
                attach_energy_request_form
                    .to_notice_use_general_energy_card_to_my_specific_unit_request(
                        opponent_unique_id,
                        generate_use_my_hand_card_data_response
                            .get_player_hand_use_map_for_notice().clone(),
                        generate_my_specific_unit_energy_data_response
                            .get_player_field_unit_energy_map_for_notice().clone())).await;

        println!("notice_response: {:?}", notice_response);

        drop(notify_player_action_info_service_guard);

        AttachGeneralEnergyCardResponseForm::from_response(
            generate_use_my_hand_card_data_response,
            generate_my_specific_unit_energy_data_response)
    }

    async fn request_to_attach_special_energy(
        &self, attach_special_energy_request_form: AttachSpecialEnergyCardRequestForm)
        -> AttachSpecialEnergyCardResponseForm {

        println!("GameCardEnergyControllerImpl: request_to_attach_special_energy()");

        // 1. 세션 아이디를 검증합니다.
        let account_unique_id = self.is_valid_session(
            attach_special_energy_request_form.to_session_validation_request()).await;

        if account_unique_id == -1 {
            return AttachSpecialEnergyCardResponseForm::default()
        }

        let energy_card_id_string = attach_special_energy_request_form.get_energy_card_id();
        let energy_card_id = energy_card_id_string.parse::<i32>().unwrap();

        // 2. GameProtocolValidation Service 호출하여 Hand 에 있는지 확인하여 해킹 여부 검증
        let mut game_protocol_validation_service_guard =
            self.game_protocol_validation_service.lock().await;

        let is_this_your_turn_response =
            game_protocol_validation_service_guard.is_this_your_turn(
                attach_special_energy_request_form
                    .to_is_this_your_turn_request(account_unique_id)).await;

        if !is_this_your_turn_response.is_success() {
            println!("당신의 턴이 아닙니다.");
            return AttachSpecialEnergyCardResponseForm::default()
        }

        let check_protocol_hacking_response =
            game_protocol_validation_service_guard.check_protocol_hacking(
                attach_special_energy_request_form
                    .to_check_protocol_hacking_request(account_unique_id, energy_card_id)).await;

        if !check_protocol_hacking_response.is_success() {
            println!("해킹범을 검거합니다!");
            return AttachSpecialEnergyCardResponseForm::default()
        }

        // 3. CardKinds Service 를 호출하여 실제 에너지 카드가 맞는지 확인
        let is_it_energy_response =
            game_protocol_validation_service_guard.is_it_energy_card(
                attach_special_energy_request_form
                    .to_is_it_energy_card_request(energy_card_id)).await;

        if !is_it_energy_response.is_success() {
            println!("에너지 카드가 아닌데 요청이 왔으므로 당신도 해킹범입니다.");
            return AttachSpecialEnergyCardResponseForm::default()
        }

        drop(game_protocol_validation_service_guard);

        // 5. Special Energy 카드 Summarized Effect 산출
        let mut game_card_energy_service_guard =
            self.game_card_energy_service.lock().await;

        let special_energy_card_effect_response =
            game_card_energy_service_guard.summary_special_energy_effect(
                attach_special_energy_request_form
                    .to_summary_special_energy_card_effect_request(energy_card_id)).await;

        drop(game_card_energy_service_guard);

        let unit_card_index_string = attach_special_energy_request_form.get_unit_card_index();
        let unit_card_index = unit_card_index_string.parse::<i32>().unwrap();

        // 6. Battle Field 유닛에 특수 에너지 붙이기
        let mut game_field_unit_service_guard =
            self.game_field_unit_service.lock().await;

        let attach_energy_to_field_unit_response =
            game_field_unit_service_guard.attach_special_energy_to_field_unit_index(
                attach_special_energy_request_form
                    .to_attach_special_energy_to_field_unit_request(
                        account_unique_id,
                        unit_card_index,
                        special_energy_card_effect_response.get_race(),
                        special_energy_card_effect_response.get_status_effect_list().to_vec())).await;

        if !attach_energy_to_field_unit_response.is_success() {
            println!("필드에 유닛에게 에너지를 부착하는 과정에서 문제가 발생하였습니다.");
            return AttachSpecialEnergyCardResponseForm::default()
        }

        let updated_energy_map_of_unit =
            game_field_unit_service_guard.get_current_attached_energy_of_field_unit_by_index(
                attach_special_energy_request_form
                    .to_get_current_attached_energy_of_unit_by_index_request(
                        account_unique_id,
                        unit_card_index)).await.get_current_attached_energy_map().clone();

        let updated_extra_effect_list_of_unit =
            game_field_unit_service_guard.acquire_unit_extra_effect(
                attach_special_energy_request_form
                    .to_acquire_unit_extra_effect_request(
                        account_unique_id,
                        unit_card_index)).await.get_extra_effect_list();

        drop(game_field_unit_service_guard);

        // 4. Hand Service 호출하여 에너지 카드 사용
        let mut game_hand_service_guard =
            self.game_hand_service.lock().await;

        let use_game_hand_unit_card_response =
            game_hand_service_guard.use_energy_card(
                attach_special_energy_request_form
                    .to_use_game_hand_energy_card_request(
                        account_unique_id, energy_card_id)).await;

        drop(game_hand_service_guard);

        let usage_hand_card_id = use_game_hand_unit_card_response.get_found_energy_card_id();

        let mut game_tomb_service_guard =
            self.game_tomb_service.lock().await;

        game_tomb_service_guard.add_used_card_to_tomb(
            attach_special_energy_request_form
                .to_place_to_tomb_request(account_unique_id, usage_hand_card_id)).await;

        drop(game_tomb_service_guard);


        let mut ui_data_generator_service_guard =
            self.ui_data_generator_service.lock().await;

        let generate_use_my_hand_card_data_response =
            ui_data_generator_service_guard.generate_use_my_hand_card_data(
                attach_special_energy_request_form
                    .to_generate_use_my_hand_card_data_request(
                        usage_hand_card_id)).await;

        let generate_my_specific_unit_energy_data_response =
            ui_data_generator_service_guard.generate_my_specific_unit_energy_data(
                attach_special_energy_request_form
                    .to_generate_my_specific_unit_energy_data_request(
                        unit_card_index,
                        updated_energy_map_of_unit)).await;

        let generate_my_specific_unit_extra_effect_data_response =
            ui_data_generator_service_guard.generate_my_specific_unit_extra_effect_data(
                attach_special_energy_request_form
                    .to_generate_my_specific_unit_extra_effect_data_request(
                        unit_card_index,
                        updated_extra_effect_list_of_unit)).await;

        drop(ui_data_generator_service_guard);

        // 7. 상대방의 고유 id 값을 확보
        let battle_room_service_guard =
            self.battle_room_service.lock().await;

        let opponent_unique_id =
            battle_room_service_guard.find_opponent_by_account_unique_id(
                attach_special_energy_request_form
                    .to_find_opponent_by_account_id_request(
                        account_unique_id)).await.get_opponent_unique_id();

        drop(battle_room_service_guard);

        // 8. 상대방에게 당신이 무엇을 했는지 알려줘야 합니다
        let mut notify_player_action_info_service_guard =
            self.notify_player_action_info_service.lock().await;

        let notice_response =
            notify_player_action_info_service_guard.notice_use_special_energy_card_to_unit(
                attach_special_energy_request_form
                    .to_notice_use_special_energy_card_to_unit_request(
                        opponent_unique_id,
                        generate_use_my_hand_card_data_response
                            .get_player_hand_use_map_for_notice().clone(),
                        generate_my_specific_unit_energy_data_response
                            .get_player_field_unit_energy_map_for_notice().clone(),
                        generate_my_specific_unit_extra_effect_data_response
                            .get_player_field_unit_extra_effect_map_for_notice().clone())).await;

        println!("notice_response: {:?}", notice_response);

        drop(notify_player_action_info_service_guard);

        AttachSpecialEnergyCardResponseForm::from_response(
            generate_use_my_hand_card_data_response,
            generate_my_specific_unit_energy_data_response,
            generate_my_specific_unit_extra_effect_data_response)
    }
}