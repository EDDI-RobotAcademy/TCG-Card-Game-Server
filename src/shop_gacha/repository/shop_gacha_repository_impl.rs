use std::collections::{HashMap, HashSet};
use std::sync::Arc;
use async_trait::async_trait;
use lazy_static::lazy_static;
use tokio::sync::Mutex as AsyncMutex;
use diesel::query_dsl::methods::{FilterDsl, FindDsl};
use diesel::result::Error;
use rand::Rng;
use crate::common::card_attributes::card_grade::card_grade_enum::GradeEnum;

use crate::shop_gacha::repository::shop_gacha_repository::ShopGachaRepository;

pub struct ShopGachaRepositoryImpl {
}

impl ShopGachaRepositoryImpl {
    pub fn new() -> Self {
        ShopGachaRepositoryImpl {
        }
    }
    pub fn get_instance() -> Arc<AsyncMutex<ShopGachaRepositoryImpl>> {
        lazy_static! {
            static ref INSTANCE: Arc<AsyncMutex<ShopGachaRepositoryImpl>> =
                Arc::new(
                    AsyncMutex::new(
                        ShopGachaRepositoryImpl::new()));
        }
        INSTANCE.clone()
    }


}

#[async_trait]
impl ShopGachaRepository for ShopGachaRepositoryImpl {
    async fn get_randomly_chosen_card_id_list(&self, how_many_cards_to_get: i32, gacha_card_list: Vec<(i32)>) -> Result<Vec<i32>, Error> {
        let mut original_card_id_list = Vec::new();
        let mut randomly_chosen_card_id_list = Vec::new();

        for card in gacha_card_list {
            original_card_id_list.push(card);
        }

        for _ in 0..how_many_cards_to_get {
            let random_index = rand::thread_rng().gen_range(0..original_card_id_list.len());
            randomly_chosen_card_id_list.push(original_card_id_list[random_index]);
        }

        println!("randomly_chosen_card_id_list : {:?}", randomly_chosen_card_id_list);

        Ok(randomly_chosen_card_id_list)
    }

    async fn get_randomly_chosen_card_id(&self, gacha_card_list: Vec<(i32, GradeEnum)>) -> i32 {
        let mut original_card_id_list = Vec::new();

        for card in gacha_card_list {
            original_card_id_list.push(card);
        }
        let random_index = rand::thread_rng().gen_range(0..original_card_id_list.len());
        original_card_id_list[random_index].0

    }
    fn apply_probability_by_grade(&self, how_many_cards_to_get: i32, is_confirmed_upper_legend: bool) -> Vec<GradeEnum> {
        let mut list_of_grade  = Vec::new();

        match is_confirmed_upper_legend {
            true => {
                let random_index = rand::thread_rng().gen_range(1..100);
                if (random_index > 90) {
                    list_of_grade.push(GradeEnum::Mythical);
                } else {
                    list_of_grade.push(GradeEnum::Legend);
                }

                for _ in 1..how_many_cards_to_get {
                    let random_index = rand::thread_rng().gen_range(1..100);
                    if (random_index > 96) {
                        list_of_grade.push(GradeEnum::Mythical);
                    } else if (random_index > 86) {
                        list_of_grade.push(GradeEnum::Legend);
                    } else if (random_index > 69) {
                        list_of_grade.push(GradeEnum::Hero);
                    } else if (random_index > 39) {
                        list_of_grade.push(GradeEnum::Uncommon);
                    } else {
                        list_of_grade.push(GradeEnum::Common);
                    }
                }
            }
            false => {
                for _ in 0..how_many_cards_to_get {
                    let random_index = rand::thread_rng().gen_range(1..100);
                    if (random_index > 96) {
                        list_of_grade.push(GradeEnum::Mythical);
                    } else if (random_index > 86) {
                        list_of_grade.push(GradeEnum::Legend);
                    } else if (random_index > 69) {
                        list_of_grade.push(GradeEnum::Hero);
                    } else if (random_index > 39) {
                        list_of_grade.push(GradeEnum::Uncommon);
                    } else {
                        list_of_grade.push(GradeEnum::Common);
                    }
                }
            }
        }

        list_of_grade
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::test;

    #[test]
    async fn test_generate_random_card_list_from_file() {
    }
}