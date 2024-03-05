use std::sync::Arc;
use async_trait::async_trait;
use lazy_static::lazy_static;
use tokio::sync::Mutex as AsyncMutex;
use crate::mulligan::repository::mulligan_repository::MulliganRepository;
use crate::mulligan::repository::mulligan_repository_impl::MulliganRepositoryImpl;

use crate::mulligan::service::mulligan_service::MulliganService;
use crate::mulligan::service::request::check_opponent_mulligan_timer_request::CheckOpponentMulliganTimerRequest;
use crate::mulligan::service::request::is_opponent_mulligan_finished_request::IsOpponentMulliganFinishedRequest;
use crate::mulligan::service::request::record_mulligan_finish_request::RecordMulliganFinishRequest;
use crate::mulligan::service::response::check_opponent_mulligan_timer_response::CheckOpponentMulliganTimerResponse;
use crate::mulligan::service::response::is_opponent_mulligan_finished_response::IsOpponentMulliganFinishedResponse;
use crate::mulligan::service::response::record_mulligan_finish_response::RecordMulliganFinishResponse;

pub struct MulliganServiceImpl {
    mulligan_repository: Arc<AsyncMutex<MulliganRepositoryImpl>>,
}

impl MulliganServiceImpl {
    pub fn new(mulligan_repository: Arc<AsyncMutex<MulliganRepositoryImpl>>) -> Self {

        MulliganServiceImpl {
            mulligan_repository
        }
    }

    pub fn get_instance() -> Arc<AsyncMutex<MulliganServiceImpl>> {
        lazy_static! {
            static ref INSTANCE: Arc<AsyncMutex<MulliganServiceImpl>> =
                Arc::new(
                    AsyncMutex::new(
                        MulliganServiceImpl::new(MulliganRepositoryImpl::get_instance())));
        }
        INSTANCE.clone()
    }
}

#[async_trait]
impl MulliganService for MulliganServiceImpl {
    async fn record_mulligan_finish(
        &self, record_mulligan_finish_request: RecordMulliganFinishRequest)
        -> RecordMulliganFinishResponse {

        println!("MulliganServiceImpl: record_mulligan_finish()");

        let mulligan_repository_guard = self.mulligan_repository.lock().await;

        let response =
            mulligan_repository_guard.record_mulligan_finish(
                record_mulligan_finish_request.get_account_unique_id()).await;

        drop(mulligan_repository_guard);

        RecordMulliganFinishResponse::new(response)
    }

    async fn is_opponent_mulligan_finished(
        &self, is_opponent_mulligan_finished_request: IsOpponentMulliganFinishedRequest)
        -> IsOpponentMulliganFinishedResponse {

        println!("MulliganServiceImpl: is_opponent_mulligan_finished()");

        let mulligan_repository_guard = self.mulligan_repository.lock().await;

        let response =
            mulligan_repository_guard.check_opponent_mulligan_finish(
                is_opponent_mulligan_finished_request.get_opponent_unique_id()).await;

        drop(mulligan_repository_guard);

        IsOpponentMulliganFinishedResponse::new(response)
    }

    async fn check_opponent_mulligan_timer(
        &self, check_opponent_mulligan_timer_request: CheckOpponentMulliganTimerRequest)
        -> CheckOpponentMulliganTimerResponse {

        println!("MulliganServiceImpl: check_opponent_mulligan_timer()");

        let mulligan_repository_guard = self.mulligan_repository.lock().await;

        let response =
            mulligan_repository_guard.check_opponent_mulligan_timer_over(
                check_opponent_mulligan_timer_request.get_opponent_unique_id()).await;

        drop(mulligan_repository_guard);

        CheckOpponentMulliganTimerResponse::new(response)
    }
}