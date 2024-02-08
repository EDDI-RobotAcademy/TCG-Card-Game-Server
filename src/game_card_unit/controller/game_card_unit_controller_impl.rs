use std::sync::Arc;
use async_trait::async_trait;
use lazy_static::lazy_static;

use tokio::sync::Mutex as AsyncMutex;
use crate::battle_room::service::battle_room_service::BattleRoomService;
use crate::battle_room::service::battle_room_service_impl::BattleRoomServiceImpl;
use crate::game_card_support::controller::response_form::energy_boost_support_response_form::EnergyBoostSupportResponseForm;
use crate::game_card_unit::controller::game_card_unit_controller::GameCardUnitController;
use crate::game_card_unit::controller::request_form::deploy_unit_request_form::DeployUnitRequestForm;
use crate::game_card_unit::controller::response_form::deploy_unit_response_form::DeployUnitResponseForm;

use crate::game_card_unit::service::game_card_unit_service_impl::GameCardUnitServiceImpl;
use crate::game_field_unit::service::game_field_unit_service::GameFieldUnitService;
use crate::game_field_unit::service::game_field_unit_service_impl::GameFieldUnitServiceImpl;
use crate::game_hand::service::game_hand_service::GameHandService;
use crate::game_hand::service::game_hand_service_impl::GameHandServiceImpl;
use crate::game_hand::service::response::legacy_use_game_hand_unit_card_response::LegacyUseGameHandUnitCardResponse;
use crate::game_protocol_validation::service::game_protocol_validation_service::GameProtocolValidationService;
use crate::game_protocol_validation::service::game_protocol_validation_service_impl::GameProtocolValidationServiceImpl;
use crate::notify_player_action::service::notify_player_action_service::NotifyPlayerActionService;
use crate::notify_player_action::service::notify_player_action_service_impl::NotifyPlayerActionServiceImpl;
use crate::redis::service::redis_in_memory_service::RedisInMemoryService;
use crate::redis::service::redis_in_memory_service_impl::RedisInMemoryServiceImpl;
use crate::redis::service::request::get_value_with_key_request::GetValueWithKeyRequest;

pub struct GameCardUnitControllerImpl {
    game_hand_service: Arc<AsyncMutex<GameHandServiceImpl>>,
    battle_room_service: Arc<AsyncMutex<BattleRoomServiceImpl>>,
    game_card_unit_service: Arc<AsyncMutex<GameCardUnitServiceImpl>>,
    game_field_unit_service: Arc<AsyncMutex<GameFieldUnitServiceImpl>>,
    redis_in_memory_service: Arc<AsyncMutex<RedisInMemoryServiceImpl>>,
    notify_player_action_service: Arc<AsyncMutex<NotifyPlayerActionServiceImpl>>,
    game_protocol_validation_service: Arc<AsyncMutex<GameProtocolValidationServiceImpl>>,
}

impl GameCardUnitControllerImpl {
    pub fn new(game_hand_service: Arc<AsyncMutex<GameHandServiceImpl>>,
               battle_room_service: Arc<AsyncMutex<BattleRoomServiceImpl>>,
               game_card_unit_service: Arc<AsyncMutex<GameCardUnitServiceImpl>>,
               game_field_unit_service: Arc<AsyncMutex<GameFieldUnitServiceImpl>>,
               redis_in_memory_service: Arc<AsyncMutex<RedisInMemoryServiceImpl>>,
               notify_player_action_service: Arc<AsyncMutex<NotifyPlayerActionServiceImpl>>,
               game_protocol_validation_service: Arc<AsyncMutex<GameProtocolValidationServiceImpl>>) -> Self {

        GameCardUnitControllerImpl {
            game_hand_service,
            battle_room_service,
            game_card_unit_service,
            game_field_unit_service,
            redis_in_memory_service,
            notify_player_action_service,
            game_protocol_validation_service
        }
    }
    pub fn get_instance() -> Arc<AsyncMutex<GameCardUnitControllerImpl>> {
        lazy_static! {
            static ref INSTANCE: Arc<AsyncMutex<GameCardUnitControllerImpl>> =
                Arc::new(
                    AsyncMutex::new(
                        GameCardUnitControllerImpl::new(
                            GameHandServiceImpl::get_instance(),
                            BattleRoomServiceImpl::get_instance(),
                            GameCardUnitServiceImpl::get_instance(),
                            GameFieldUnitServiceImpl::get_instance(),
                            RedisInMemoryServiceImpl::get_instance(),
                            NotifyPlayerActionServiceImpl::get_instance(),
                            GameProtocolValidationServiceImpl::get_instance())));
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
impl GameCardUnitController for GameCardUnitControllerImpl {
    async fn request_to_deploy_unit(&self, deploy_unit_request_form: DeployUnitRequestForm) -> DeployUnitResponseForm {
        println!("GameCardUnitControllerImpl: request_to_deploy_unit()");

        // 1. 세션 아이디를 검증합니다.
        let account_unique_id = self.is_valid_session(deploy_unit_request_form.to_session_validation_request()).await;
        if account_unique_id == -1 {
            return DeployUnitResponseForm::new(false)
        }

        // TODO: 세션을 제외하고 애초에 UI에서 숫자로 전송하면 더 좋다.
        let unit_id_string = deploy_unit_request_form.get_unit_id();
        let unit_card_id = unit_id_string.parse::<i32>().unwrap();

        // 2. GameProtocolValidation Service 호출하여 Hand에 있는지 확인하여 해킹 여부 검증
        let mut game_protocol_validation_service_guard = self.game_protocol_validation_service.lock().await;
        let check_protocol_hacking_response = game_protocol_validation_service_guard.check_protocol_hacking(
            deploy_unit_request_form.to_check_protocol_hacking_request(account_unique_id, unit_card_id)).await;
        if !check_protocol_hacking_response.is_success() {
            println!("해킹범을 검거합니다!");
            return DeployUnitResponseForm::new(false)
        }

        // 3. CardKinds Service를 호출하여 실제 유닛 카드가 맞는지 확인
        let is_it_unit_response = game_protocol_validation_service_guard.is_it_unit_card(
            deploy_unit_request_form.to_is_it_unit_card_request(unit_card_id)).await;
        if !is_it_unit_response.is_success() {
            println!("유닛 카드가 아닌데 요청이 왔으므로 당신도 해킹범입니다.");
            return DeployUnitResponseForm::new(false)
        }

        // 4. Hand Service 호출하여 카드 사용
        let mut game_hand_service_guard = self.game_hand_service.lock().await;
        let use_game_hand_unit_card_response = game_hand_service_guard.use_unit_card(
            deploy_unit_request_form.to_use_game_hand_unit_card_request(account_unique_id, unit_card_id)).await;
        let usage_hand_card_id = use_game_hand_unit_card_response.get_found_unit_card_id();

        // 5. Battle Field에 유닛 배치
        let mut game_field_unit_service_guard = self.game_field_unit_service.lock().await;
        let add_unit_to_game_field_response = game_field_unit_service_guard.add_unit_to_game_field(
            deploy_unit_request_form.to_add_unit_to_game_field_request(account_unique_id, usage_hand_card_id)).await;
        if !add_unit_to_game_field_response.is_success() {
            println!("필드에 유닛 배치 중 문제가 발생하였습니다.");
            return DeployUnitResponseForm::new(false)
        }

        // 6. 상대방의 고유 id 값을 확보
        let battle_room_service_guard = self.battle_room_service.lock().await;
        let find_opponent_by_account_id_response = battle_room_service_guard.find_opponent_by_account_unique_id(
            deploy_unit_request_form.to_find_opponent_by_account_id_request(account_unique_id)).await;

        // 7. 상대방에게 당신이 무엇을 했는지 알려줘야 합니다
        let mut notify_player_action_service_guard = self.notify_player_action_service.lock().await;
        let notify_to_opponent_what_you_do_response = notify_player_action_service_guard.notify_to_opponent_what_you_do(
            deploy_unit_request_form.to_notify_to_opponent_what_you_do_request(
                find_opponent_by_account_id_response.get_opponent_unique_id(), usage_hand_card_id)).await;
        if !notify_to_opponent_what_you_do_response.is_success() {
            println!("상대에게 무엇을 했는지 알려주는 과정에서 문제가 발생했습니다.");
            return DeployUnitResponseForm::new(false)
        }

        DeployUnitResponseForm::new(true)
    }
}
