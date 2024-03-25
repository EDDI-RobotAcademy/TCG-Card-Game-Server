use std::sync::Arc;
use async_trait::async_trait;
use lazy_static::lazy_static;

use tokio::sync::Mutex as AsyncMutex;
use crate::battle_room::service::battle_room_service::BattleRoomService;
use crate::battle_room::service::battle_room_service_impl::BattleRoomServiceImpl;
use crate::common::card_attributes::card_race::card_race_enum::RaceEnum;
use crate::common::message::false_message_enum::FalseMessage::{NotYourTurn, NotYourTurnFieldEnergy};
use crate::game_field_energy::controller::game_field_energy_controller::GameFieldEnergyController;
use crate::game_field_energy::controller::request_form::attach_field_energy_to_field_unit_request_form::AttachFieldEnergyToFieldUnitRequestForm;
use crate::game_field_energy::controller::response_form::attach_field_energy_to_field_unit_response_form::AttachFieldEnergyToFieldUnitResponseForm;
use crate::game_field_energy::service::game_field_energy_service::GameFieldEnergyService;
use crate::game_field_energy::service::game_field_energy_service_impl::GameFieldEnergyServiceImpl;
use crate::game_field_unit::service::game_field_unit_service::GameFieldUnitService;
use crate::game_field_unit::service::game_field_unit_service_impl::GameFieldUnitServiceImpl;
use crate::game_protocol_validation::service::game_protocol_validation_service::GameProtocolValidationService;
use crate::game_protocol_validation::service::game_protocol_validation_service_impl::GameProtocolValidationServiceImpl;
use crate::notify_player_action_info::service::notify_player_action_info_service::NotifyPlayerActionInfoService;
use crate::notify_player_action_info::service::notify_player_action_info_service_impl::NotifyPlayerActionInfoServiceImpl;
use crate::redis::service::redis_in_memory_service::RedisInMemoryService;
use crate::redis::service::redis_in_memory_service_impl::RedisInMemoryServiceImpl;
use crate::redis::service::request::get_value_with_key_request::GetValueWithKeyRequest;
use crate::ui_data_generator::service::ui_data_generator_service::UiDataGeneratorService;
use crate::ui_data_generator::service::ui_data_generator_service_impl::UiDataGeneratorServiceImpl;

pub struct GameFieldEnergyControllerImpl {
    game_field_energy_service: Arc<AsyncMutex<GameFieldEnergyServiceImpl>>,
    battle_room_service: Arc<AsyncMutex<BattleRoomServiceImpl>>,
    game_field_unit_service: Arc<AsyncMutex<GameFieldUnitServiceImpl>>,
    redis_in_memory_service: Arc<AsyncMutex<RedisInMemoryServiceImpl>>,
    notify_player_action_info_service: Arc<AsyncMutex<NotifyPlayerActionInfoServiceImpl>>,
    game_protocol_validation_service: Arc<AsyncMutex<GameProtocolValidationServiceImpl>>,
    ui_data_generator_service: Arc<AsyncMutex<UiDataGeneratorServiceImpl>>,
}

impl GameFieldEnergyControllerImpl {
    pub fn new(game_field_energy_service: Arc<AsyncMutex<GameFieldEnergyServiceImpl>>,
               battle_room_service: Arc<AsyncMutex<BattleRoomServiceImpl>>,
               game_field_unit_service: Arc<AsyncMutex<GameFieldUnitServiceImpl>>,
               redis_in_memory_service: Arc<AsyncMutex<RedisInMemoryServiceImpl>>,
               notify_player_action_info_service: Arc<AsyncMutex<NotifyPlayerActionInfoServiceImpl>>,
               game_protocol_validation_service: Arc<AsyncMutex<GameProtocolValidationServiceImpl>>,
               ui_data_generator_service: Arc<AsyncMutex<UiDataGeneratorServiceImpl>>,) -> Self {

        GameFieldEnergyControllerImpl {
            game_field_energy_service,
            battle_room_service,
            game_field_unit_service,
            redis_in_memory_service,
            notify_player_action_info_service,
            game_protocol_validation_service,
            ui_data_generator_service,
        }
    }
    pub fn get_instance() -> Arc<AsyncMutex<GameFieldEnergyControllerImpl>> {
        lazy_static! {
            static ref INSTANCE: Arc<AsyncMutex<GameFieldEnergyControllerImpl>> =
                Arc::new(
                    AsyncMutex::new(
                        GameFieldEnergyControllerImpl::new(
                            GameFieldEnergyServiceImpl::get_instance(),
                            BattleRoomServiceImpl::get_instance(),
                            GameFieldUnitServiceImpl::get_instance(),
                            RedisInMemoryServiceImpl::get_instance(),
                            NotifyPlayerActionInfoServiceImpl::get_instance(),
                            GameProtocolValidationServiceImpl::get_instance(),
                            UiDataGeneratorServiceImpl::get_instance())));
        }
        INSTANCE.clone()
    }

    async fn is_valid_session(&self, request: GetValueWithKeyRequest) -> i32 {
        let redis_in_memory_service_guard = self.redis_in_memory_service.lock().await;
        let session_validation_response = redis_in_memory_service_guard.get_value_with_key(request).await;

        let value_string = session_validation_response.get_value();
        value_string.parse::<i32>().unwrap_or_else(|_| { -1 })
    }
}

#[async_trait]
impl GameFieldEnergyController for GameFieldEnergyControllerImpl {
    async fn request_to_attach_field_energy_to_field_unit(
        &self, attach_field_energy_to_field_unit_request_form: AttachFieldEnergyToFieldUnitRequestForm)
        -> AttachFieldEnergyToFieldUnitResponseForm {

        println!("GameFieldEnergyControllerImpl: request_to_attach_field_energy_to_field_unit()");

        // 1. 세션 아이디를 검증합니다.
        let account_unique_id = self.is_valid_session(
            attach_field_energy_to_field_unit_request_form.to_session_validation_request()).await;

        if account_unique_id == -1 {
            println!("세션이 존재하지 않습니다.");
            return AttachFieldEnergyToFieldUnitResponseForm::default()
        }

        // 2. 플레이어의 턴을 검증합니다.
        let mut game_protocol_validation_service_guard =
            self.game_protocol_validation_service.lock().await;

        let is_this_your_turn_response =
            game_protocol_validation_service_guard.is_this_your_turn(
                attach_field_energy_to_field_unit_request_form
                    .to_is_this_your_turn_request(account_unique_id)).await;

        if !is_this_your_turn_response.is_success() {
            println!("당신의 턴이 아닙니다.");
            return AttachFieldEnergyToFieldUnitResponseForm::from_response_with_message(NotYourTurnFieldEnergy)
        }

        drop(game_protocol_validation_service_guard);

        // 3. 필드 에너지가 충분히 있는지 검증합니다.
        let will_be_used_field_energy_quantity_string =
            attach_field_energy_to_field_unit_request_form.get_quantity().to_string();
        let will_be_used_field_energy_quantity =
            will_be_used_field_energy_quantity_string.parse::<i32>().unwrap();

        let mut game_field_energy_service_guard =
            self.game_field_energy_service.lock().await;

        let check_field_energy_enough_to_use_response =
            game_field_energy_service_guard.check_field_energy_enough_to_use(
                attach_field_energy_to_field_unit_request_form
                    .to_check_field_energy_enough_to_use_request(
                        account_unique_id,
                        will_be_used_field_energy_quantity)).await;

        if !check_field_energy_enough_to_use_response.is_success() {
            println!("필드 에너지가 사용하기에 충분하지 않습니다.");
            return AttachFieldEnergyToFieldUnitResponseForm::default()
        }

        // 4. 필드 에너지를 수량에 따라 부착합니다.
        let unit_card_index_string =
            attach_field_energy_to_field_unit_request_form.get_unit_index().to_string();
        let unit_card_index = unit_card_index_string.parse::<i32>().unwrap();

        let energy_race_string =
            attach_field_energy_to_field_unit_request_form.get_energy_race().to_string();
        let energy_race_enum = RaceEnum::from(energy_race_string.parse::<i32>().unwrap());

        let mut game_field_unit_service_guard =
            self.game_field_unit_service.lock().await;

        println!("{:?} 에너지 {}개를 부착합니다.", energy_race_enum, will_be_used_field_energy_quantity);
        let attach_multiple_energy_to_unit_response =
            game_field_unit_service_guard.attach_multiple_energy_to_field_unit_index(
                attach_field_energy_to_field_unit_request_form
                    .to_attach_multiple_energy_to_unit_index_request(
                        account_unique_id,
                        unit_card_index,
                        energy_race_enum,
                        will_be_used_field_energy_quantity)).await;

        if !attach_multiple_energy_to_unit_response.is_success() {
            println!("필드 에너지를 부착하는 과정에서 문제가 발생하였습니다.");
            return AttachFieldEnergyToFieldUnitResponseForm::default()
        }

        // 5. 에너지를 부착한 유닛의 업데이트 된 에너지 정보를 가져옵니다.
        let updated_energy_map =
            game_field_unit_service_guard.get_current_attached_energy_of_field_unit_by_index(
                attach_field_energy_to_field_unit_request_form
                    .to_get_current_attached_energy_of_field_unit_by_index_request(
                        account_unique_id,
                        unit_card_index)).await.get_current_attached_energy_map().clone();

        // 6. 사용된 필드 에너지를 제거합니다.
        game_field_energy_service_guard.remove_field_energy_with_amount(
            attach_field_energy_to_field_unit_request_form
                .to_remove_field_energy_with_amount_request(
                    account_unique_id,
                    will_be_used_field_energy_quantity)).await;

        // 7. 남아 있는 필드 에너지 정보를 가져옵니다.
        let remaining_field_energy_count =
            game_field_energy_service_guard.get_current_field_energy(
                attach_field_energy_to_field_unit_request_form
                    .to_get_current_field_energy_request(
                        account_unique_id)).await.get_field_energy_count();

        drop(game_field_unit_service_guard);
        drop(game_field_energy_service_guard);

        // 8. UI 로 전송할 데이터를 가공합니다.
        let mut ui_data_generator_service_guard =
            self.ui_data_generator_service.lock().await;

        let generate_my_field_energy_data_response =
            ui_data_generator_service_guard.generate_my_field_energy_data(
                attach_field_energy_to_field_unit_request_form
                    .to_generate_my_field_energy_data_request(remaining_field_energy_count)).await;

        let generate_my_specific_unit_energy_data_response =
            ui_data_generator_service_guard.generate_my_specific_unit_energy_data(
                attach_field_energy_to_field_unit_request_form
                    .to_generate_my_specific_unit_energy_data_request(
                        unit_card_index,
                        updated_energy_map)).await;

        drop(ui_data_generator_service_guard);

        // 9. 상대방의 고유 아이디를 가져옵니다.
        let mut battle_room_service_guard =
            self.battle_room_service.lock().await;

        let opponent_unique_id =
            battle_room_service_guard.find_opponent_by_account_unique_id(
                attach_field_energy_to_field_unit_request_form
                    .to_find_opponent_by_account_id_request(
                        account_unique_id)).await.get_opponent_unique_id();

        drop(battle_room_service_guard);

        // 10. 필드 에너지 사용에 따른 변화를 상대방에게 알립니다.
        let mut notify_player_action_info_service_guard =
            self.notify_player_action_info_service.lock().await;

        let notice_response =
            notify_player_action_info_service_guard.notice_use_field_energy_to_my_specific_unit(
                attach_field_energy_to_field_unit_request_form
                    .to_notice_use_field_energy_to_my_specific_unit_request(
                        opponent_unique_id,
                        generate_my_field_energy_data_response
                            .get_player_field_energy_map_for_notice().clone(),
                        generate_my_specific_unit_energy_data_response
                            .get_player_field_unit_energy_map_for_notice().clone())).await;

        println!("notice_response: {:?}", notice_response);

        drop(notify_player_action_info_service_guard);

        // 11. 응답을 반환합니다.
        AttachFieldEnergyToFieldUnitResponseForm::from_response(
            generate_my_field_energy_data_response,
            generate_my_specific_unit_energy_data_response)
    }
}