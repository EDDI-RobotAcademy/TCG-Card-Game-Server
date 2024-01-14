use std::sync::Arc;
use async_trait::async_trait;
use lazy_static::lazy_static;
use tokio::sync::Mutex as AsyncMutex;
use crate::account::repository::account_repository::AccountRepository;
use crate::account::repository::account_repository_impl::AccountRepositoryImpl;
use crate::account::service::account_service::AccountService;
use crate::account::service::request::account_register_request::AccountRegisterRequest;
use crate::receiver::service::server_receiver_service::ServerReceiverService;
use crate::receiver::service::server_receiver_service_impl::ServerReceiverServiceImpl;


pub struct AccountServiceImpl {
    repository: Arc<AsyncMutex<AccountRepositoryImpl>>,
}

impl AccountServiceImpl {
    pub fn new(repository:
               Arc<AsyncMutex<AccountRepositoryImpl>>) -> Self {
        AccountServiceImpl {
            repository,
        }
    }

    pub fn get_instance() -> Arc<AsyncMutex<AccountServiceImpl>> {
        lazy_static! {
            static ref INSTANCE: Arc<AsyncMutex<AccountServiceImpl>> =
                Arc::new(
                    AsyncMutex::new(
                        AccountServiceImpl::new(
                            AccountRepositoryImpl::get_instance())));
        }
        INSTANCE.clone()
    }
}

#[async_trait]
impl AccountService for AccountServiceImpl {
    async fn account_register(&self, account_register_request: AccountRegisterRequest) {
        println!("AccountServiceImpl: account_register()");

        let account_repository = self.repository.lock().await;

        if let Ok(account) = account_register_request.to_account() {
            account_repository.save(account).await;
        } else {
            eprintln!("계정 생성 중 에러 발생!");
        }
    }
}