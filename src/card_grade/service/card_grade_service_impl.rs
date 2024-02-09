use std::collections::HashMap;
use std::sync::Arc;
use async_trait::async_trait;
use lazy_static::lazy_static;

use tokio::sync::Mutex as AsyncMutex;
use crate::card_grade::repository::card_grade_repository::CardGradeRepository;
use crate::card_grade::repository::card_grade_repository_impl::CardGradeRepositoryImpl;
use crate::card_grade::service::card_grade_service::CardGradeService;
use crate::common::card_attributes::card_grade::card_grade_enum::GradeEnum;

pub struct CardGradeServiceImpl {
    card_grade_repository: Arc<AsyncMutex<CardGradeRepositoryImpl>>
}

impl CardGradeServiceImpl {
    pub fn new(card_grade_repository: Arc<AsyncMutex<CardGradeRepositoryImpl>>) -> Self {
        CardGradeServiceImpl {
            card_grade_repository
        }
    }
    pub fn get_instance() -> Arc<AsyncMutex<CardGradeServiceImpl>> {
        lazy_static! {
            static ref INSTANCE: Arc<AsyncMutex<CardGradeServiceImpl>> =
                Arc::new(
                    AsyncMutex::new(
                        CardGradeServiceImpl::new(
                            CardGradeRepositoryImpl::get_instance())));
        }
        INSTANCE.clone()
    }
}

#[async_trait]
impl CardGradeService for CardGradeServiceImpl {
    async fn get_card_grade(&self, card_number: &i32) -> GradeEnum {
        println!("CardGradeServiceImpl: get_card_grade()");

        let card_grade_repository_guard = self.card_grade_repository.lock().await;
        card_grade_repository_guard.get_card_grade(card_number).await
    }
    async fn get_cards_below_target_grade(&self, card_list: Vec<i32>, target_grade: GradeEnum) -> Vec<i32> {
        println!("CardGradeServiceImpl: get_cards_below_target_grade()");

        let card_grade_repository_guard = self.card_grade_repository.lock().await;
        let card_grade_map = card_grade_repository_guard.get_grade_by_card_list(card_list).await;
        let mut result_vector = Vec::new();
        for (card_id, grade_enum) in card_grade_map {
            if grade_enum as i32 <= target_grade as i32 {
                result_vector.push(card_id)
            }
        }
        result_vector
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::test;

    #[test]
    async fn test_get_card_grade() {
        let card_grade_repo_mutex = CardGradeServiceImpl::get_instance();
        let card_grade_repo_guard = card_grade_repo_mutex.lock().await;
        let card_number: i32 = 6;

        let card_grade = card_grade_repo_guard.get_card_grade(&card_number).await;

        println!("Card Grade: {:?}", card_grade);
        assert_eq!(card_grade, GradeEnum::Common);
    }
    #[test]
    async fn test_get_cards_below_target_grade() {
        let card_list = vec![25, 8, 36, 19, 2, 26, 35, 93, 32, 31, 30, 151, 20, 33, 9, 27];
        let target_grade = GradeEnum::Hero;

        let card_grade_service_mutex = CardGradeServiceImpl::get_instance();
        let card_grade_service_mutex_guard = card_grade_service_mutex.lock().await;

        let card_list_below_hero_grade = card_grade_service_mutex_guard.get_cards_below_target_grade(card_list, target_grade).await;

        assert_eq!(card_list_below_hero_grade.len(), 12);
        println!("Cards below hero grade : {:?}", card_list_below_hero_grade)
    }
}