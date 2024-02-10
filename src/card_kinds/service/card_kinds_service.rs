use async_trait::async_trait;
use crate::card_kinds::service::request::pick_specific_kind_on_the_card_list_request::PickSpecificKindOnTheCardListRequest;
use crate::card_kinds::service::response::pick_specific_kind_on_the_card_list_response::PickSpecificKindOnTheCardListResponse;
use crate::common::card_attributes::card_kinds::card_kinds_enum::KindsEnum;

#[async_trait]
pub trait CardKindsService {
    async fn get_card_kind(&self, card_number: &i32) -> KindsEnum;
    async fn pick_specific_kind_on_the_card_list(
        &self, pick_specific_kind_on_the_card_list_request: PickSpecificKindOnTheCardListRequest) -> PickSpecificKindOnTheCardListResponse;
}