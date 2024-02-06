use std::sync::Arc;
use async_trait::async_trait;
use lazy_static::lazy_static;

use tokio::sync::Mutex as AsyncMutex;
use crate::card_grade::repository::card_grade_repository::CardGradeRepository;
use crate::card_grade::repository::card_grade_repository_impl::CardGradeRepositoryImpl;
use crate::card_grade::service::card_grade_service::CardGradeService;

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
    async fn get_card_grade(&self, card_number: &i32) -> Option<i32> {
        println!("CardGradeServiceImpl: get_card_grade()");

        let card_grade_repository_guard = self.card_grade_repository.lock().await;
        card_grade_repository_guard.get_card_grade(card_number).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::test;

    #[test]
    async fn test_get_card_kind() {
        let card_grade_repo_mutex = CardGradeServiceImpl::get_instance();
        let card_grade_repo_guard = card_grade_repo_mutex.lock().await;
        let card_number: i32 = 6;

        let card_grade = card_grade_repo_guard.get_card_grade(&card_number).await;

        match card_grade {
            Some(grade) => {
                println!("Card Grade: {}", grade);
                assert_eq!(grade, 1);
            }
            None => println!("Card not found."),
        }
    }
}