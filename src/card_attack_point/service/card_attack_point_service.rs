use async_trait::async_trait;

use crate::common::card_attributes::card_attack_point::attack_point_enum::AttackPointEnum;

#[async_trait]
pub trait CardAttackPointService {
    async fn get_card_attack_point(&self, card_number: &i32) -> i32;
}