use std::sync::Arc;
use async_trait::async_trait;
use lazy_static::lazy_static;

use tokio::sync::Mutex as AsyncMutex;
use crate::game_card_energy::repository::game_card_energy_repository::GameCardEnergyRepository;
use crate::game_card_energy::repository::game_card_energy_repository_impl::GameCardEnergyRepositoryImpl;

use crate::game_card_energy::service::game_card_energy_service::GameCardEnergyService;
use crate::game_card_energy::service::request::summary_energy_card_effect_request::SummaryEnergyCardEffectRequest;
use crate::game_card_energy::service::request::summary_special_energy_card_effect_request::SummarySpecialEnergyCardEffectRequest;
use crate::game_card_energy::service::response::summary_energy_card_effect_response::SummaryEnergyCardEffectResponse;
use crate::game_card_energy::service::response::summary_special_energy_card_effect_response::SummarySpecialEnergyCardEffectResponse;

pub struct GameCardEnergyServiceImpl {
    game_card_energy_repository: Arc<AsyncMutex<GameCardEnergyRepositoryImpl>>,
}

impl GameCardEnergyServiceImpl {
    pub fn new(game_card_energy_repository: Arc<AsyncMutex<GameCardEnergyRepositoryImpl>>,
    ) -> Self {
        GameCardEnergyServiceImpl {
            game_card_energy_repository,
        }
    }

    pub fn get_instance() -> Arc<AsyncMutex<GameCardEnergyServiceImpl>> {
        lazy_static! {
            static ref INSTANCE: Arc<AsyncMutex<GameCardEnergyServiceImpl>> =
                Arc::new(
                    AsyncMutex::new(
                        GameCardEnergyServiceImpl::new(
                            GameCardEnergyRepositoryImpl::get_instance())));
        }
        INSTANCE.clone()
    }
}

#[async_trait]
impl GameCardEnergyService for GameCardEnergyServiceImpl {
    async fn summary_energy_effect(&mut self, summary_energy_card_effect_request: SummaryEnergyCardEffectRequest) -> SummaryEnergyCardEffectResponse {
        println!("GameCardEnergyServiceImpl: use_energy_card()");

        let game_card_energy_repository_guard = self.game_card_energy_repository.lock().await;
        let summary_energy_card_effect_response = unsafe {
            game_card_energy_repository_guard.call_energy_card_repository_handler(summary_energy_card_effect_request.get_energy_card_id())
        };

        return SummaryEnergyCardEffectResponse::from_summary_energy_card_effect(summary_energy_card_effect_response)
    }

    async fn summary_special_energy_effect(&mut self, summary_special_energy_card_effect_request: SummarySpecialEnergyCardEffectRequest) -> SummarySpecialEnergyCardEffectResponse {
        println!("GameCardEnergyServiceImpl: summary_special_energy_effect()");

        let game_card_energy_repository_guard = self.game_card_energy_repository.lock().await;
        let summary_special_energy_card_effect_response = unsafe {
            game_card_energy_repository_guard.call_energy_card_repository_handler(
                summary_special_energy_card_effect_request.get_special_energy_card_id())
        };

        return SummarySpecialEnergyCardEffectResponse::from_summary_special_energy_card_effect(summary_special_energy_card_effect_response)
    }
}