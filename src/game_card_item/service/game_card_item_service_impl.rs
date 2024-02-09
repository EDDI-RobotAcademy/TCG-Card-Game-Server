use std::sync::Arc;
use async_trait::async_trait;
use lazy_static::lazy_static;

use tokio::sync::Mutex as AsyncMutex;
use crate::game_card_energy::service::response::summary_energy_card_effect_response::SummaryEnergyCardEffectResponse;

use crate::game_card_item::entity::game_card_item_effect::GameCardItemEffect;
use crate::game_card_item::repository::game_card_item_repository::GameCardItemRepository;

use crate::game_card_item::repository::game_card_item_repository_impl::GameCardItemRepositoryImpl;
use crate::game_card_item::service::game_card_item_service::GameCardItemService;
use crate::game_card_item::service::request::calculate_item_effect_request::CalculateItemEffectRequest;
use crate::game_card_item::service::request::summary_item_card_effect_request::SummaryItemCardEffectRequest;
use crate::game_card_item::service::response::summary_item_card_effect_response::SummaryItemCardEffectResponse;

pub struct GameCardItemServiceImpl {
    game_card_item_repository: Arc<AsyncMutex<GameCardItemRepositoryImpl>>,
}

impl GameCardItemServiceImpl {
    pub fn new(game_card_item_repository: Arc<AsyncMutex<GameCardItemRepositoryImpl>>) -> Self {
        GameCardItemServiceImpl {
            game_card_item_repository,
        }
    }

    pub fn get_instance() -> Arc<AsyncMutex<GameCardItemServiceImpl>> {
        lazy_static! {
            static ref INSTANCE: Arc<AsyncMutex<GameCardItemServiceImpl>> =
                Arc::new(
                    AsyncMutex::new(
                        GameCardItemServiceImpl::new(
                            GameCardItemRepositoryImpl::get_instance())));
        }
        INSTANCE.clone()
    }
}

#[async_trait]
impl GameCardItemService for GameCardItemServiceImpl {
    async fn use_item_card(&mut self, calculate_item_effect_request: CalculateItemEffectRequest) -> GameCardItemEffect {
        println!("GameCardItemServiceImpl: use_item_card()");

        let game_card_item_service_guard = self.game_card_item_repository.lock().await;
        let game_card_item_effect = unsafe {
            game_card_item_service_guard.call_item_card_repository_handler(calculate_item_effect_request.get_item_card_number())
        };

        return game_card_item_effect
    }

    async fn summary_item_card(&mut self, summary_item_card_effect_request: SummaryItemCardEffectRequest) -> SummaryItemCardEffectResponse {
        println!("GameCardEnergyServiceImpl: summary_item_card()");

        let game_card_item_repository_guard = self.game_card_item_repository.lock().await;
        let summary_item_card_effect_response = unsafe {
            game_card_item_repository_guard.call_item_card_repository_handler(summary_item_card_effect_request.get_item_card_id())
        };

        return SummaryItemCardEffectResponse::from_summary_item_card_effect(summary_item_card_effect_response)
    }
}