use async_trait::async_trait;
use crate::common::card_attributes::card_kinds::card_kinds_enum::KindsEnum;

#[async_trait]
pub trait CardKindsService {
    async fn get_card_kind(&self, card_number: &i32) -> KindsEnum;
}