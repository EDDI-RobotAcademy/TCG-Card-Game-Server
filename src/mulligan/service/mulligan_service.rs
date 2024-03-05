use async_trait::async_trait;

use crate::mulligan::service::request::check_opponent_mulligan_timer_request::CheckOpponentMulliganTimerRequest;
use crate::mulligan::service::request::is_opponent_mulligan_finished_request::IsOpponentMulliganFinishedRequest;
use crate::mulligan::service::request::record_mulligan_finish_request::RecordMulliganFinishRequest;

use crate::mulligan::service::response::check_opponent_mulligan_timer_response::CheckOpponentMulliganTimerResponse;
use crate::mulligan::service::response::is_opponent_mulligan_finished_response::IsOpponentMulliganFinishedResponse;
use crate::mulligan::service::response::record_mulligan_finish_response::RecordMulliganFinishResponse;

#[async_trait]
pub trait MulliganService {
    async fn record_mulligan_finish(
        &self, record_mulligan_finish_request: RecordMulliganFinishRequest)
        -> RecordMulliganFinishResponse;
    async fn is_opponent_mulligan_finished(
        &self, is_opponent_mulligan_finished_request: IsOpponentMulliganFinishedRequest)
        -> IsOpponentMulliganFinishedResponse;
    async fn check_opponent_mulligan_timer(
        &self, check_opponent_mulligan_timer_request: CheckOpponentMulliganTimerRequest)
        -> CheckOpponentMulliganTimerResponse;
}