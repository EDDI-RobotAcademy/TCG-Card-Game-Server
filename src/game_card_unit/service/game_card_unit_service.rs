use async_trait::async_trait;

use crate::game_card_unit::service::request::summary_unit_card_info_request::SummaryUnitCardInfoRequest;
use crate::game_card_unit::service::request::summary_unit_card_passive_default_request::SummaryUnitCardPassiveDefaultRequest;
use crate::game_card_unit::service::response::summary_unit_card_info_response::SummaryUnitCardInfoResponse;
use crate::game_card_unit::service::response::summary_unit_card_passive_default_response::SummaryUnitCardPassiveDefaultResponse;

#[async_trait]
pub trait GameCardUnitService {
    async fn summary_unit_card(&mut self, summary_unit_card_info_request: SummaryUnitCardInfoRequest) -> SummaryUnitCardInfoResponse;
    async fn summary_unit_card_passive_default(&mut self, summary_unit_card_passive_default_request: SummaryUnitCardPassiveDefaultRequest)
        -> SummaryUnitCardPassiveDefaultResponse;
}
