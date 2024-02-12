use std::sync::Arc;
use async_trait::async_trait;
use lazy_static::lazy_static;
use tokio::sync::Mutex as AsyncMutex;

use crate::game_card_support::entity::game_card_support_effect::GameCardSupportEffect;
use crate::game_card_support::repository::game_card_support_repository::GameCardSupportRepository;
use crate::game_card_support::repository::game_card_support_repository_impl::GameCardSupportRepositoryImpl;
use crate::game_card_support::service::game_card_support_service::GameCardSupportService;
use crate::game_card_support::service::request::summarize_support_card_effect_request::SummarizeSupportCardEffectRequest;

pub struct GameCardSupportServiceImpl {
    game_card_support_repository: Arc<AsyncMutex<GameCardSupportRepositoryImpl>>,
}

impl GameCardSupportServiceImpl {
    pub fn new(game_card_support_repository: Arc<AsyncMutex<GameCardSupportRepositoryImpl>>) -> Self {
        GameCardSupportServiceImpl {
            game_card_support_repository,
        }
    }

    pub fn get_instance() -> Arc<AsyncMutex<GameCardSupportServiceImpl>> {
        lazy_static! {
            static ref INSTANCE: Arc<AsyncMutex<GameCardSupportServiceImpl>> =
                Arc::new(
                    AsyncMutex::new(
                        GameCardSupportServiceImpl::new(
                            GameCardSupportRepositoryImpl::get_instance())));
        }
        INSTANCE.clone()
    }
}

#[async_trait]
impl GameCardSupportService for GameCardSupportServiceImpl {
    async fn summarize_support_card_effect(&mut self, summarize_support_card_effect_request: SummarizeSupportCardEffectRequest) -> GameCardSupportEffect {
        println!("GameCardSupportServiceImpl: use_support_card()");

        let game_card_support_repository_guard = self.game_card_support_repository.lock().await;
        let game_card_support_effect = unsafe {
            game_card_support_repository_guard.call_support_card_repository_handler(summarize_support_card_effect_request.get_support_card_number())
        };

        return game_card_support_effect
    }
}
