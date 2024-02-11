use std::sync::Arc;
use async_trait::async_trait;
use lazy_static::lazy_static;

use tokio::sync::Mutex as AsyncMutex;
use crate::card_kinds::repository::card_kinds_repository::CardKindsRepository;
use crate::card_kinds::repository::card_kinds_repository_impl::CardKindsRepositoryImpl;
use crate::card_kinds::service::card_kinds_service::CardKindsService;
use crate::card_kinds::service::request::pick_specific_kind_on_the_card_list_request::PickSpecificKindOnTheCardListRequest;
use crate::card_kinds::service::response::pick_specific_kind_on_the_card_list_response::PickSpecificKindOnTheCardListResponse;
use crate::common::card_attributes::card_kinds::card_kinds_enum::KindsEnum;

pub struct CardKindsServiceImpl {
    card_kinds_repository: Arc<AsyncMutex<CardKindsRepositoryImpl>>
}

impl CardKindsServiceImpl {
    pub fn new(card_kinds_repository: Arc<AsyncMutex<CardKindsRepositoryImpl>>) -> Self {
        CardKindsServiceImpl {
            card_kinds_repository
        }
    }
    pub fn get_instance() -> Arc<AsyncMutex<CardKindsServiceImpl>> {
        lazy_static! {
            static ref INSTANCE: Arc<AsyncMutex<CardKindsServiceImpl>> =
                Arc::new(
                    AsyncMutex::new(
                        CardKindsServiceImpl::new(
                            CardKindsRepositoryImpl::get_instance())));
        }
        INSTANCE.clone()
    }
}

#[async_trait]
impl CardKindsService for CardKindsServiceImpl {
    async fn get_card_kind(&self, card_number: &i32) -> KindsEnum {
        println!("CardKindsServiceImpl: get_card_kind()");

        let card_kinds_repository_guard = self.card_kinds_repository.lock().await;
        card_kinds_repository_guard.get_card_kind(card_number).await
    }
    async fn pick_specific_kind_on_the_card_list(
        &self, pick_specific_kind_on_the_card_list_request: PickSpecificKindOnTheCardListRequest) -> PickSpecificKindOnTheCardListResponse {
        println!("CardKindsServiceImpl: pick_specific_kind_on_the_card_list()");

        let card_list = pick_specific_kind_on_the_card_list_request.get_card_list().clone();
        let target_kind = pick_specific_kind_on_the_card_list_request.get_specific_kind().clone();

        let card_kinds_repository_guard = self.card_kinds_repository.lock().await;
        let card_kind_map = card_kinds_repository_guard.get_card_kind_map_of_card_list(card_list).await;
        let mut result_vector = Vec::new();
        for (card_id, kind_enum) in card_kind_map {
            if kind_enum == target_kind {
                result_vector.push(card_id)
            }
        }

        PickSpecificKindOnTheCardListResponse::new(result_vector)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_get_card_kind() {
        let card_kinds_service_mutex = CardKindsServiceImpl::get_instance();
        let card_kinds_service_guard = card_kinds_service_mutex.lock().await;
        let card_number: i32 = 2;

        let card_kind = card_kinds_service_guard.get_card_kind(&card_number).await;
        assert_eq!(card_kind, KindsEnum::Support);
        println!("Card Kind: {:?}", card_kind);
    }

    #[tokio::test]
    async fn test_pick_specific_kind_on_the_card_list() {
        let card_kinds_service_mutex = CardKindsServiceImpl::get_instance();
        let card_kinds_service_guard = card_kinds_service_mutex.lock().await;

        let card_list = vec![25, 8, 36, 19, 2, 26, 35, 93, 32, 31, 30, 151, 20, 33, 9, 27];
        let target_kind = KindsEnum::Unit;

        let request = PickSpecificKindOnTheCardListRequest::new(card_list, target_kind);

        let response = card_kinds_service_guard.pick_specific_kind_on_the_card_list(request).await;

        println!("Unit card on the list: {:?}", response.get_filtered_card_list());
    }
}

