use std::sync::Arc;
use async_trait::async_trait;
use lazy_static::lazy_static;

use tokio::sync::Mutex as AsyncMutex;
use crate::battle_room::service::battle_room_service_impl::BattleRoomServiceImpl;
use crate::common::card_attributes::card_race::card_race_enum::RaceEnum;
use crate::game_card_energy::controller::response_form::attach_general_energy_card_response_form::AttachGeneralEnergyCardResponseForm;
use crate::game_card_energy::service::game_card_energy_service_impl::GameCardEnergyServiceImpl;
use crate::game_field_energy::controller::game_field_energy_controller::GameFieldEnergyController;
use crate::game_field_energy::controller::request_form::attach_field_energy_to_field_unit_request_form::AttachFieldEnergyToFieldUnitRequestForm;
use crate::game_field_energy::controller::response_form::attach_field_energy_to_field_unit_response_form::AttachFieldEnergyToFieldUnitResponseForm;
use crate::game_field_energy::service::game_field_energy_service::GameFieldEnergyService;
use crate::game_field_energy::service::game_field_energy_service_impl::GameFieldEnergyServiceImpl;
use crate::game_field_unit::service::game_field_unit_service::GameFieldUnitService;
use crate::game_field_unit::service::game_field_unit_service_impl::GameFieldUnitServiceImpl;
use crate::game_hand::service::game_hand_service_impl::GameHandServiceImpl;
use crate::game_protocol_validation::service::game_protocol_validation_service_impl::GameProtocolValidationServiceImpl;
use crate::notify_player_action::service::notify_player_action_service_impl::NotifyPlayerActionServiceImpl;
use crate::redis::service::redis_in_memory_service::RedisInMemoryService;
use crate::redis::service::redis_in_memory_service_impl::RedisInMemoryServiceImpl;
use crate::redis::service::request::get_value_with_key_request::GetValueWithKeyRequest;

pub struct GameFieldEnergyControllerImpl {
    game_field_energy_service: Arc<AsyncMutex<GameFieldEnergyServiceImpl>>,
    game_hand_service: Arc<AsyncMutex<GameHandServiceImpl>>,
    game_field_unit_service: Arc<AsyncMutex<GameFieldUnitServiceImpl>>,
    redis_in_memory_service: Arc<AsyncMutex<RedisInMemoryServiceImpl>>,
    notify_player_action_service: Arc<AsyncMutex<NotifyPlayerActionServiceImpl>>,
    game_protocol_validation_service: Arc<AsyncMutex<GameProtocolValidationServiceImpl>>,
}

impl GameFieldEnergyControllerImpl {
    pub fn new(game_field_energy_service: Arc<AsyncMutex<GameFieldEnergyServiceImpl>>,
               game_hand_service: Arc<AsyncMutex<GameHandServiceImpl>>,
               game_field_unit_service: Arc<AsyncMutex<GameFieldUnitServiceImpl>>,
               redis_in_memory_service: Arc<AsyncMutex<RedisInMemoryServiceImpl>>,
               notify_player_action_service: Arc<AsyncMutex<NotifyPlayerActionServiceImpl>>,
               game_protocol_validation_service: Arc<AsyncMutex<GameProtocolValidationServiceImpl>>) -> Self {

        GameFieldEnergyControllerImpl {
            game_field_energy_service,
            game_hand_service,
            game_field_unit_service,
            redis_in_memory_service,
            notify_player_action_service,
            game_protocol_validation_service,
        }
    }
    pub fn get_instance() -> Arc<AsyncMutex<GameFieldEnergyControllerImpl>> {
        lazy_static! {
            static ref INSTANCE: Arc<AsyncMutex<GameFieldEnergyControllerImpl>> =
                Arc::new(
                    AsyncMutex::new(
                        GameFieldEnergyControllerImpl::new(
                            GameFieldEnergyServiceImpl::get_instance(),
                            GameHandServiceImpl::get_instance(),
                            GameFieldUnitServiceImpl::get_instance(),
                            RedisInMemoryServiceImpl::get_instance(),
                            NotifyPlayerActionServiceImpl::get_instance(),
                            GameProtocolValidationServiceImpl::get_instance())));
        }
        INSTANCE.clone()
    }

    async fn is_valid_session(&self, request: GetValueWithKeyRequest) -> i32 {
        let mut redis_in_memory_service_guard = self.redis_in_memory_service.lock().await;
        let session_validation_response = redis_in_memory_service_guard.get_value_with_key(request).await;

        let value_string = session_validation_response.get_value();
        value_string.parse::<i32>().unwrap_or_else(|_| { -1 })
    }
}

#[async_trait]
impl GameFieldEnergyController for GameFieldEnergyControllerImpl {
    async fn request_to_attach_field_energy_to_field_unit(
        &self, attach_field_energy_to_field_unit_request_form: AttachFieldEnergyToFieldUnitRequestForm) -> AttachFieldEnergyToFieldUnitResponseForm {
        println!("GameFieldEnergyControllerImpl: request_to_attach_field_energy_to_field_unit()");

        // 1. 세션 아이디를 검증합니다.
        let account_unique_id = self.is_valid_session(attach_field_energy_to_field_unit_request_form.to_session_validation_request()).await;
        if account_unique_id == -1 {
            return AttachFieldEnergyToFieldUnitResponseForm::new(false)
        }

        // 2. TODO: 플레이어의 턴 검증

        // 3. 필드 에너지가 충분히 있는지 검증합니다.
        let will_be_used_field_energy_quantity_string = attach_field_energy_to_field_unit_request_form.get_quantity().to_string();
        let will_be_used_field_energy_quantity = will_be_used_field_energy_quantity_string.parse::<i32>().unwrap();

        let mut game_field_energy_service_guard = self.game_field_energy_service.lock().await;
        let check_field_energy_enough_to_use_response = game_field_energy_service_guard
            .check_field_energy_enough_to_use(
                attach_field_energy_to_field_unit_request_form
                    .to_check_field_energy_enough_to_use_request(account_unique_id,
                                                                 will_be_used_field_energy_quantity)).await;

        if !check_field_energy_enough_to_use_response.is_success() {
            println!("필드 에너지가 사용하기에 충분하지 않습니다.");
            return AttachFieldEnergyToFieldUnitResponseForm::new(false)
        }

        drop(game_field_energy_service_guard);

        // 4. 필드 에너지를 수량에 따라 부착합니다.
        let unit_card_index_string = attach_field_energy_to_field_unit_request_form.get_unit_index().to_string();
        let unit_card_index = unit_card_index_string.parse::<i32>().unwrap();

        let energy_race_string = attach_field_energy_to_field_unit_request_form.get_energy_race().to_string();
        let energy_race_enum = RaceEnum::from(energy_race_string.parse::<i32>().unwrap());

        let mut game_field_unit_service_guard = self.game_field_unit_service.lock().await;

        if will_be_used_field_energy_quantity == 1 {
            let attach_single_energy_to_unit_index_response = game_field_unit_service_guard
                .attach_energy_to_field_unit_index(
                    attach_field_energy_to_field_unit_request_form
                        .to_attach_single_energy_to_unit_index_request(account_unique_id, unit_card_index, energy_race_enum)).await;

            if !attach_single_energy_to_unit_index_response.is_success() {
                println!("필드 에너지를 부착하는 과정에서 문제가 발생하였습니다.");
                return AttachFieldEnergyToFieldUnitResponseForm::new(false)
            }
        }

        let attach_multiple_energy_to_unit_response = game_field_unit_service_guard
            .attach_multiple_energy_to_field_unit_index(
                attach_field_energy_to_field_unit_request_form
                    .to_attach_multiple_energy_to_unit_index_request(account_unique_id,
                                                                     unit_card_index,
                                                                     energy_race_enum,
                                                                     will_be_used_field_energy_quantity)).await;

        if !attach_multiple_energy_to_unit_response.is_success() {
            println!("필드 에너지를 부착하는 과정에서 문제가 발생하였습니다.");
            return AttachFieldEnergyToFieldUnitResponseForm::new(false)
        }

        drop(game_field_unit_service_guard);

        // 5. TODO: 필드 에너지 사용에 따른 변화를 상대방에게 알립니다.

        AttachFieldEnergyToFieldUnitResponseForm::new(true)
    }
}