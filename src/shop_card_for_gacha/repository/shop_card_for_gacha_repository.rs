use std::collections::HashMap;
use async_trait::async_trait;
use crate::common::card_attributes::card_grade::card_grade_enum::GradeEnum;
use crate::common::card_attributes::card_race::card_race_enum::RaceEnum;

#[async_trait]
pub trait ShopCardForGachaRepository {
    async fn get_specific_race_card_list(&self , race_value: RaceEnum) -> HashMap<i32, GradeEnum>;
}