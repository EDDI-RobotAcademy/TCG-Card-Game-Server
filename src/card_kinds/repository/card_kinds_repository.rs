use std::collections::HashMap;
use async_trait::async_trait;
use crate::common::card_attributes::card_kinds::card_kinds_enum::KindsEnum;

#[async_trait]
pub trait CardKindsRepository {
    async fn get_card_kind(&self, card_number: &i32) -> KindsEnum;
    async fn get_card_kind_map_of_card_list(&self, card_list: Vec<i32>) -> HashMap<i32, KindsEnum>;
}