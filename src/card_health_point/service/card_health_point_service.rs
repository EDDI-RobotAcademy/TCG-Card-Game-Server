use async_trait::async_trait;

use crate::common::card_attributes::card_health_point::health_point_enum::HealthPointEnum;


#[async_trait]
pub trait CardHealthPointService {
    async fn get_card_health_point(&self, card_number: &i32) -> i32;
}