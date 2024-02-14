use std::sync::Arc;
use async_trait::async_trait;
use lazy_static::lazy_static;

use tokio::sync::Mutex as AsyncMutex;

use crate::game_card_tool::entity::game_card_tool_effect::GameCardToolEffect;
use crate::game_card_tool::repository::game_card_tool_repository::GameCardToolRepository;
use crate::game_card_tool::repository::game_card_tool_repository_impl::GameCardToolRepositoryImpl;
use crate::game_card_tool::service::game_card_tool_service::GameCardToolService;
use crate::game_card_tool::service::request::summarize_tool_card_effect_request::SummarizeToolCardEffectRequest;
use crate::redis::repository::redis_in_memory_repository::RedisInMemoryRepository;

pub struct GameCardToolServiceImpl {
    game_card_tool_repository: Arc<AsyncMutex<GameCardToolRepositoryImpl>>,
}

impl GameCardToolServiceImpl {
    pub fn new(game_card_tool_repository: Arc<AsyncMutex<GameCardToolRepositoryImpl>>,
    ) -> Self {
        GameCardToolServiceImpl {
            game_card_tool_repository,
        }
    }

    pub fn get_instance() -> Arc<AsyncMutex<GameCardToolServiceImpl>> {
        lazy_static! {
            static ref INSTANCE: Arc<AsyncMutex<GameCardToolServiceImpl>> =
                Arc::new(
                    AsyncMutex::new(
                        GameCardToolServiceImpl::new(
                            GameCardToolRepositoryImpl::get_instance())));
        }
        INSTANCE.clone()
    }
}

#[async_trait]
impl GameCardToolService for GameCardToolServiceImpl {
    async fn summarize_tool_card_effect(&mut self, summarize_tool_card_effect_request: SummarizeToolCardEffectRequest) -> GameCardToolEffect {
        println!("GameCardToolServiceImpl: summarize_tool_card_effect()");

        let game_card_tool_repository_guard = self.game_card_tool_repository.lock().await;
        let game_card_tool_effect = unsafe {
            game_card_tool_repository_guard.call_tool_card_repository_handler(summarize_tool_card_effect_request.get_tool_card_id())
        };

        return game_card_tool_effect
    }
}
