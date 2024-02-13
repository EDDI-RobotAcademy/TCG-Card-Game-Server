use std::sync::Arc;
use async_trait::async_trait;
use lazy_static::lazy_static;

use tokio::sync::Mutex as AsyncMutex;
use crate::game_card_active_skill::repository::game_card_active_skill_repository::GameCardActiveSkillRepository;

use crate::game_card_active_skill::repository::game_card_active_skill_repository_impl::GameCardActiveSkillRepositoryImpl;
use crate::game_card_active_skill::service::game_card_active_skill_service::GameCardActiveSkillService;
use crate::game_card_active_skill::service::request::summary_active_skill_effect_request::SummaryActiveSkillEffectRequest;
use crate::game_card_active_skill::service::response::summary_active_skill_effect_response::SummaryActiveSkillEffectResponse;
use crate::game_card_item::service::response::summary_item_card_effect_response::SummaryItemCardEffectResponse;

pub struct GameCardActiveSkillServiceImpl {
    game_card_active_skill_repository: Arc<AsyncMutex<GameCardActiveSkillRepositoryImpl>>,
}

impl GameCardActiveSkillServiceImpl {
    pub fn new(game_card_active_skill_repository: Arc<AsyncMutex<GameCardActiveSkillRepositoryImpl>>) -> Self {
        GameCardActiveSkillServiceImpl {
            game_card_active_skill_repository,
        }
    }

    pub fn get_instance() -> Arc<AsyncMutex<GameCardActiveSkillServiceImpl>> {
        lazy_static! {
            static ref INSTANCE: Arc<AsyncMutex<GameCardActiveSkillServiceImpl>> =
                Arc::new(
                    AsyncMutex::new(
                        GameCardActiveSkillServiceImpl::new(
                            GameCardActiveSkillRepositoryImpl::get_instance())));
        }
        INSTANCE.clone()
    }
}

#[async_trait]
impl GameCardActiveSkillService for GameCardActiveSkillServiceImpl {
    async fn summary_active_skill(&mut self, summary_active_skill_effect_request: SummaryActiveSkillEffectRequest) -> SummaryActiveSkillEffectResponse {
        println!("GameCardActiveSkillServiceImpl: summary_active_skill()");

        let game_card_active_skill_repository_guard = self.game_card_active_skill_repository.lock().await;
        let summary_active_skill_effect_response = unsafe {
            game_card_active_skill_repository_guard.call_active_skill_repository_handler(
                summary_active_skill_effect_request.get_active_skill_usage_card_id(),
                summary_active_skill_effect_request.get_usage_skill_index())
        };

        return SummaryActiveSkillEffectResponse::from_summary_active_skill_effect(summary_active_skill_effect_response)
    }
}