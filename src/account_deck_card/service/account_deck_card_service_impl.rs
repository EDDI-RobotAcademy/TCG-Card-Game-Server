use std::sync::Arc;
use async_trait::async_trait;
use lazy_static::lazy_static;
use tokio::sync::Mutex as AsyncMutex;

use crate::card_kinds::repository::card_kinds_repository::CardKindsRepository;
use crate::card_kinds::repository::card_kinds_repository_impl::CardKindsRepositoryImpl;

use crate::account_deck_card::repository::account_deck_card_repository::AccountDeckCardRepository;

use crate::account_deck_card::repository::account_deck_card_repository_impl::AccountDeckCardRepositoryImpl;
use crate::account_deck_card::service::account_deck_card_service::AccountDeckCardService;
use crate::account_deck_card::service::request::account_deck_card_list_request::AccountDeckCardListRequest;
use crate::account_deck_card::service::request::account_deck_configuration_request::AccountDeckConfigurationRequest;
use crate::account_deck_card::service::response::account_deck_card_list_response::AccountDeckCardListResponse;
use crate::account_deck_card::service::response::account_deck_configuration_response::AccountDeckConfigurationResponse;

pub struct AccountDeckCardServiceImpl {
    deck_card_repository: Arc<AsyncMutex<AccountDeckCardRepositoryImpl>>,
    card_kinds_repository: Arc<AsyncMutex<CardKindsRepositoryImpl>>,
}

impl AccountDeckCardServiceImpl {
    pub fn new(deck_card_repository: Arc<AsyncMutex<AccountDeckCardRepositoryImpl>>,
               card_kinds_repository: Arc<AsyncMutex<CardKindsRepositoryImpl>>,) -> Self {
        AccountDeckCardServiceImpl {
            deck_card_repository,
            card_kinds_repository,
        }
    }
    pub fn get_instance() -> Arc<AsyncMutex<AccountDeckCardServiceImpl>> {
        lazy_static! {
            static ref INSTANCE: Arc<AsyncMutex<AccountDeckCardServiceImpl >> =
                Arc::new(
                    AsyncMutex::new(
                        AccountDeckCardServiceImpl::new(
                            AccountDeckCardRepositoryImpl::get_instance(),
                            CardKindsRepositoryImpl::get_instance())));
        }
        INSTANCE.clone()
    }
}

#[async_trait]
impl AccountDeckCardService for AccountDeckCardServiceImpl {
    async fn deck_configuration_register(&self, deck_configuration_request: AccountDeckConfigurationRequest) -> AccountDeckConfigurationResponse {
        println!("AccountDeckCardServiceImpl: deck_configuration_register()");

        let deck_card_vector = deck_configuration_request.to_deck_card_list();
        let deck_card_repository = self.deck_card_repository.lock().await;
        let result = deck_card_repository.save_deck_card_list(deck_card_vector).await;
        match result {
            Ok(success_message) => {
                AccountDeckConfigurationResponse::new(true, success_message)
            }
            Err(error_message) => {
                AccountDeckConfigurationResponse::new(false, error_message)
            }
        }
    }
    async fn deck_card_list(&self, deck_card_list_request: AccountDeckCardListRequest) -> AccountDeckCardListResponse {
        println!("AccountDeckCardServiceImpl: deck_card_list()");

        let deck_card_repository = self.deck_card_repository.lock().await;
        let deck_id_i32 = deck_card_list_request.deck_id();
        let result = deck_card_repository.get_card_list(deck_id_i32).await;
        match result {
            Ok(opt_list) => {
                let card_id_count_list = opt_list.unwrap();
                AccountDeckCardListResponse::new(card_id_count_list)
            }
            Err(e) => {
                let empty_list = Vec::new();
                AccountDeckCardListResponse::new(empty_list)
            }
        }
    }
}