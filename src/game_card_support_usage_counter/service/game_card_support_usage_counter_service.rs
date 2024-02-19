use async_trait::async_trait;

use crate::game_card_support_usage_counter::service::request::check_support_card_usage_count_request::CheckSupportCardUsageCountRequest;
use crate::game_card_support_usage_counter::service::request::reset_support_card_usage_count_request::ResetSupportCardUsageCountRequest;
use crate::game_card_support_usage_counter::service::request::update_support_card_usage_count_request::UpdateSupportCardUsageCountRequest;
use crate::game_card_support_usage_counter::service::response::check_support_card_usage_count_response::CheckSupportCardUsageCountResponse;
use crate::game_card_support_usage_counter::service::response::reset_support_card_usage_count_response::ResetSupportCardUsageCountResponse;
use crate::game_card_support_usage_counter::service::response::update_support_card_usage_count_response::UpdateSupportCardUsageCountResponse;

#[async_trait]
pub trait GameCardSupportUsageCounterService {
    async fn reset_support_card_usage_count(
        &mut self, reset_support_card_usage_count_request: ResetSupportCardUsageCountRequest)
        -> ResetSupportCardUsageCountResponse;

    async fn check_support_card_usage_count(
        &mut self, check_support_card_usage_count_request: CheckSupportCardUsageCountRequest)
        -> CheckSupportCardUsageCountResponse;

    async fn update_support_card_usage_count(
        &mut self, update_support_card_usage_count_request: UpdateSupportCardUsageCountRequest)
        -> UpdateSupportCardUsageCountResponse;
}