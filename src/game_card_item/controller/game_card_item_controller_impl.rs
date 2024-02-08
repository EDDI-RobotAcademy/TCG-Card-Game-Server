use std::sync::Arc;
use async_trait::async_trait;
use lazy_static::lazy_static;

use tokio::sync::Mutex as AsyncMutex;
use crate::game_card_item::controller::game_card_item_controller::GameCardItemController;
use crate::game_card_item::controller::request_form::target_death_item_request_form::TargetDeathItemRequestForm;
use crate::game_card_item::controller::response_form::target_death_item_response_form::TargetDeathItemResponseForm;

use crate::game_card_item::service::game_card_item_service_impl::GameCardItemServiceImpl;
use crate::game_protocol_validation::service::game_protocol_validation_service_impl::GameProtocolValidationServiceImpl;
use crate::redis::service::redis_in_memory_service_impl::RedisInMemoryServiceImpl;

pub struct GameCardItemControllerImpl {
    game_card_item_service: Arc<AsyncMutex<GameCardItemServiceImpl>>,
    game_protocol_validation_service: Arc<AsyncMutex<GameProtocolValidationServiceImpl>>,
    redis_in_memory_service: Arc<AsyncMutex<RedisInMemoryServiceImpl>>,
}

impl GameCardItemControllerImpl {
    pub fn new(game_card_item_service: Arc<AsyncMutex<GameCardItemServiceImpl>>,
               game_protocol_validation_service: Arc<AsyncMutex<GameProtocolValidationServiceImpl>>,
               redis_in_memory_service: Arc<AsyncMutex<RedisInMemoryServiceImpl>>, ) -> Self {

        GameCardItemControllerImpl {
            game_card_item_service,
            game_protocol_validation_service,
            redis_in_memory_service,
        }
    }
    pub fn get_instance() -> Arc<AsyncMutex<GameCardItemControllerImpl>> {
        lazy_static! {
            static ref INSTANCE: Arc<AsyncMutex<GameCardItemControllerImpl>> =
                Arc::new(
                    AsyncMutex::new(
                        GameCardItemControllerImpl::new(
                            GameCardItemServiceImpl::get_instance(),
                            GameProtocolValidationServiceImpl::get_instance(),
                            RedisInMemoryServiceImpl::get_instance())));
        }
        INSTANCE.clone()
    }
}

#[async_trait]
impl GameCardItemController for GameCardItemControllerImpl {
    async fn request_to_use_target_death_item(&self, target_death_item_request_form: TargetDeathItemRequestForm) -> TargetDeathItemResponseForm {
        println!("GameCardItemControllerImpl: request_to_use_target_death_item()");

        TargetDeathItemResponseForm::new(true)
    }
}