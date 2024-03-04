use std::sync::Arc;
use async_trait::async_trait;
use lazy_static::lazy_static;

use tokio::sync::Mutex as AsyncMutex;
use crate::game_card_active_skill::service::response::summary_active_skill_effect_response::SummaryActiveSkillEffectResponse;
use crate::game_card_passive_skill::repository::game_card_passive_skill_repository::GameCardPassiveSkillRepository;

use crate::game_card_passive_skill::repository::game_card_passive_skill_repository_impl::GameCardPassiveSkillRepositoryImpl;
use crate::game_card_passive_skill::service::game_card_passive_skill_service::GameCardPassiveSkillService;
use crate::game_card_passive_skill::service::request::summary_passive_skill_effect_by_index_request::SummaryPassiveSkillEffectByIndexRequest;
use crate::game_card_passive_skill::service::request::summary_passive_skill_effect_request::SummaryPassiveSkillEffectRequest;
use crate::game_card_passive_skill::service::response::summary_passive_skill_effect_by_index_response::SummaryPassiveSkillEffectByIndexResponse;
use crate::game_card_passive_skill::service::response::summary_passive_skill_effect_response::SummaryPassiveSkillEffectResponse;
use crate::game_card_unit::service::response::summary_unit_card_info_response::SummaryUnitCardInfoResponse;

pub struct GameCardPassiveSkillServiceImpl {
    game_card_passive_skill_repository: Arc<AsyncMutex<GameCardPassiveSkillRepositoryImpl>>,
}

impl GameCardPassiveSkillServiceImpl {
    pub fn new(game_card_passive_skill_repository: Arc<AsyncMutex<GameCardPassiveSkillRepositoryImpl>>,
    ) -> Self {
        GameCardPassiveSkillServiceImpl {
            game_card_passive_skill_repository,
        }
    }

    pub fn get_instance() -> Arc<AsyncMutex<GameCardPassiveSkillServiceImpl>> {
        lazy_static! {
            static ref INSTANCE: Arc<AsyncMutex<GameCardPassiveSkillServiceImpl>> =
                Arc::new(
                    AsyncMutex::new(
                        GameCardPassiveSkillServiceImpl::new(
                            GameCardPassiveSkillRepositoryImpl::get_instance())));
        }
        INSTANCE.clone()
    }
}

#[async_trait]
impl GameCardPassiveSkillService for GameCardPassiveSkillServiceImpl {
    // summary_passive_skill_effect_request: SummaryPassiveSkillEffectRequest
    async fn summary_passive_skill(&self, summary_passive_skill_effect_request: SummaryPassiveSkillEffectRequest) -> SummaryPassiveSkillEffectResponse {
        println!("GameCardPassiveSkillServiceImpl: summary_passive_skill()");

        let game_card_passive_skill_repository_guard = self.game_card_passive_skill_repository.lock().await;
        let summary_first_passive_skill_effect_response = unsafe {
            game_card_passive_skill_repository_guard
                .call_passive_skill_repository_handler(
                    summary_passive_skill_effect_request.get_unit_card_id(),
                    1)
        };

        let summary_second_passive_skill_effect_response = unsafe {
            game_card_passive_skill_repository_guard
                .call_passive_skill_repository_handler(
                    summary_passive_skill_effect_request.get_unit_card_id(),
                    2)
        };

        let summary_third_passive_skill_effect_response = unsafe {
            game_card_passive_skill_repository_guard
                .call_passive_skill_repository_handler(
                    summary_passive_skill_effect_request.get_unit_card_id(),
                    3)
        };

        return SummaryPassiveSkillEffectResponse::new(
            summary_first_passive_skill_effect_response,
            summary_second_passive_skill_effect_response,
            summary_third_passive_skill_effect_response)
    }
    async fn summary_deploy_passive_skill(&self, summary_deploy_passive_skill_effect_request: SummaryPassiveSkillEffectRequest) -> SummaryPassiveSkillEffectResponse {
        println!("GameCardPassiveSkillServiceImpl: summary_deploy_passive_skill()");

        let game_card_passive_skill_repository_guard = self.game_card_passive_skill_repository.lock().await;
        let summary_first_passive_skill_effect_response = unsafe {
            game_card_passive_skill_repository_guard
                .call_deploy_passive_skill_repository_handler(
                    summary_deploy_passive_skill_effect_request.get_unit_card_id(),
                    1)
        };

        let summary_second_passive_skill_effect_response = unsafe {
            game_card_passive_skill_repository_guard
                .call_deploy_passive_skill_repository_handler(
                    summary_deploy_passive_skill_effect_request.get_unit_card_id(),
                    2)
        };

        let summary_third_passive_skill_effect_response = unsafe {
            game_card_passive_skill_repository_guard
                .call_deploy_passive_skill_repository_handler(
                    summary_deploy_passive_skill_effect_request.get_unit_card_id(),
                    3)
        };

        return SummaryPassiveSkillEffectResponse::new(
            summary_first_passive_skill_effect_response,
            summary_second_passive_skill_effect_response,
            summary_third_passive_skill_effect_response)
    }
    async fn summary_turn_start_passive_skill(&self, summary_deploy_passive_skill_effect_request: SummaryPassiveSkillEffectRequest) -> SummaryPassiveSkillEffectResponse {
        println!("GameCardPassiveSkillServiceImpl: summary_turn_start_passive_skill()");

        let game_card_passive_skill_repository_guard = self.game_card_passive_skill_repository.lock().await;
        let summary_first_passive_skill_effect_response = unsafe {
            game_card_passive_skill_repository_guard
                .call_turn_start_passive_skill_repository_handler(
                    summary_deploy_passive_skill_effect_request.get_unit_card_id(),
                    1)
        };

        let summary_second_passive_skill_effect_response = unsafe {
            game_card_passive_skill_repository_guard
                .call_turn_start_passive_skill_repository_handler(
                    summary_deploy_passive_skill_effect_request.get_unit_card_id(),
                    2)
        };

        let summary_third_passive_skill_effect_response = unsafe {
            game_card_passive_skill_repository_guard
                .call_turn_start_passive_skill_repository_handler(
                    summary_deploy_passive_skill_effect_request.get_unit_card_id(),
                    3)
        };

        return SummaryPassiveSkillEffectResponse::new(
            summary_first_passive_skill_effect_response,
            summary_second_passive_skill_effect_response,
            summary_third_passive_skill_effect_response)
    }
    async fn summary_passive_skill_by_index(
        &self, summary_passive_skill_effect_by_index_request: SummaryPassiveSkillEffectByIndexRequest) -> SummaryPassiveSkillEffectByIndexResponse {
        println!("GameCardPassiveSkillServiceImpl: summary_passive_skill()");

        let game_card_passive_skill_repository_guard = self.game_card_passive_skill_repository.lock().await;
        let summary_passive_skill_effect_response = unsafe {
            game_card_passive_skill_repository_guard.call_passive_skill_repository_handler(
                summary_passive_skill_effect_by_index_request.get_unit_card_id(),
                summary_passive_skill_effect_by_index_request.get_usage_skill_index())
        };

        return SummaryPassiveSkillEffectByIndexResponse::from_summary_passive_skill_effect(summary_passive_skill_effect_response)
    }
}