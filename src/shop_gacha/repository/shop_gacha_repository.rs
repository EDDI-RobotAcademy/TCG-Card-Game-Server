use std::collections::HashMap;
use async_trait::async_trait;
use diesel::result::Error;
use crate::common::card_attributes::card_grade::card_grade_enum::GradeEnum;

#[async_trait]
pub trait ShopGachaRepository {
    async fn get_randomly_chosen_card_id_list(&self, how_many_cards_to_get: i32, gacha_card_list: Vec<(i32)>) -> Result<Vec<i32>, Error> ;

    async fn get_randomly_chosen_card_id(&self, gacha_card_list: Vec<(i32, GradeEnum)>) -> i32 ;

    fn apply_probability_by_grade(&self, how_many_cards_to_get: i32, is_confirmed_upper_legend: bool) -> Vec<GradeEnum> ;
}