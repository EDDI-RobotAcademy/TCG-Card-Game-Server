use std::sync::Arc;
use async_trait::async_trait;
use lazy_static::lazy_static;

use tokio::sync::Mutex as AsyncMutex;

use crate::account::service::account_service::AccountService;
use crate::account::service::account_service_impl::AccountServiceImpl;
use crate::fake_battle_room::controller::fake_battle_room_controller::FakeBattleRoomController;
use crate::fake_battle_room::controller::request_form::create_fake_battle_room_request_form::CreateFakeBattleRoomRequestForm;
use crate::fake_battle_room::controller::response_form::create_fake_battle_room_response_form::CreateFakeBattleRoomResponseForm;
use crate::fake_battle_room::service::fake_battle_room_service::FakeBattleRoomService;
use crate::fake_battle_room::service::fake_battle_room_service_impl::FakeBattleRoomServiceImpl;
use crate::game_card_unit::controller::response_form::deploy_unit_response_form::DeployUnitResponseForm;
use crate::redis::service::redis_in_memory_service::RedisInMemoryService;
use crate::redis::service::redis_in_memory_service_impl::RedisInMemoryServiceImpl;
use crate::redis::service::request::get_value_with_key_request::GetValueWithKeyRequest;

pub struct FakeBattleRoomControllerImpl {
    account_service: Arc<AsyncMutex<AccountServiceImpl>>,
    redis_in_memory_service: Arc<AsyncMutex<RedisInMemoryServiceImpl>>,
    fake_battle_room_service: Arc<AsyncMutex<FakeBattleRoomServiceImpl>>,
}

impl FakeBattleRoomControllerImpl {
    pub fn new(account_service: Arc<AsyncMutex<AccountServiceImpl>>,
               redis_in_memory_service: Arc<AsyncMutex<RedisInMemoryServiceImpl>>,
               fake_battle_room_service: Arc<AsyncMutex<FakeBattleRoomServiceImpl>>) -> Self {

        FakeBattleRoomControllerImpl {
            account_service,
            redis_in_memory_service,
            fake_battle_room_service,
        }
    }
    pub fn get_instance() -> Arc<AsyncMutex<FakeBattleRoomControllerImpl>> {
        lazy_static! {
            static ref INSTANCE: Arc<AsyncMutex<FakeBattleRoomControllerImpl>> =
                Arc::new(
                    AsyncMutex::new(
                        FakeBattleRoomControllerImpl::new(
                            AccountServiceImpl::get_instance(),
                            RedisInMemoryServiceImpl::get_instance(),
                            FakeBattleRoomServiceImpl::get_instance())));
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
impl FakeBattleRoomController for FakeBattleRoomControllerImpl {
    async fn request_to_create_fake_battle_room(
        &self,
        create_fake_battle_room_request_form: CreateFakeBattleRoomRequestForm)
            -> CreateFakeBattleRoomResponseForm {

        let account_service_guard = self.account_service.lock().await;
        let first_login_response =
            account_service_guard.account_login(
                create_fake_battle_room_request_form.to_first_fake_test_login_request()).await;

        let second_login_response =
            account_service_guard.account_login(
                create_fake_battle_room_request_form.to_second_fake_test_login_request()).await;

        let fake_your_session = first_login_response.get_redis_token();
        let fake_opponent_session = second_login_response.get_redis_token();

        let fake_your_unique_id =
            self.is_valid_session(
                create_fake_battle_room_request_form
                    .to_session_validation_request(
                        fake_your_session)).await;

        if fake_your_unique_id == -1 {
            return CreateFakeBattleRoomResponseForm::new("".to_string(), "".to_string());
        }

        let fake_opponent_unique_id =
            self.is_valid_session(
                create_fake_battle_room_request_form
                    .to_session_validation_request(
                        fake_opponent_session)).await;

        if fake_opponent_unique_id == -1 {
            return CreateFakeBattleRoomResponseForm::new("".to_string(), "".to_string());
        }

        let fake_battle_room_service_guard = self.fake_battle_room_service.lock().await;
        let create_fake_battle_room_response =
            fake_battle_room_service_guard.create_fake_battle_room(
                create_fake_battle_room_request_form
                    .to_create_fake_battle_room_request(
                        fake_your_unique_id,
                        fake_opponent_unique_id)).await;

        return CreateFakeBattleRoomResponseForm::from_login_response(
            first_login_response,
            second_login_response)
    }
}
