use async_trait::async_trait;
use crate::account_point::entity::account_point::AccountPoint;

use crate::account_point::service::request::gain_gold_request::GainGoldRequest;
use crate::account_point::service::request::pay_gold_request::PayGoldRequest;

use crate::account_point::service::response::gain_gold_response::GainGoldResponse;
use crate::account_point::service::response::pay_gold_response::PayGoldResponse;

#[async_trait]
pub trait AccountPointService {
    async fn gain_gold(&self, gain_gold_request: GainGoldRequest ) -> GainGoldResponse;
    async fn pay_gold(&self, pay_gold_response: PayGoldRequest ) -> PayGoldResponse;
    async fn find_by_account_id(&self, account_unique_id: i32 ) -> AccountPoint;
    async fn update_event_check(&self, account_unique_id: i32);
}