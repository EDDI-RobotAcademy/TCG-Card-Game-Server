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

pub struct FakeBattleRoomControllerImpl {
    account_service: Arc<AsyncMutex<AccountServiceImpl>>,
    fake_battle_room_service: Arc<AsyncMutex<FakeBattleRoomServiceImpl>>,
}

impl FakeBattleRoomControllerImpl {
    pub fn new(account_service: Arc<AsyncMutex<AccountServiceImpl>>,
               fake_battle_room_service: Arc<AsyncMutex<FakeBattleRoomServiceImpl>>) -> Self {

        FakeBattleRoomControllerImpl {
            account_service,
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
                            FakeBattleRoomServiceImpl::get_instance())));
        }
        INSTANCE.clone()
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

        let fake_battle_room_service_guard = self.fake_battle_room_service.lock().await;
        let create_fake_battle_room_response =
            fake_battle_room_service_guard.create_fake_battle_room(
                create_fake_battle_room_request_form.to_create_fake_battle_room_request()).await;

        return CreateFakeBattleRoomResponseForm::from_login_response(
            first_login_response,
            second_login_response)
    }
}
