use std::sync::Arc;
use async_trait::async_trait;
use lazy_static::lazy_static;
use tokio::sync::Mutex as AsyncMutex;
use crate::account::repository::account_repository::AccountRepository;
use crate::account::repository::account_repository_impl::AccountRepositoryImpl;
use crate::account::service::account_service::AccountService;
use crate::account::service::request::account_register_request::AccountRegisterRequest;
use crate::account::service::response::account_register_response::AccountRegisterResponse;



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
    async fn account_register(&self, account_register_request: AccountRegisterRequest) -> AccountRegisterResponse {
        println!("AccountServiceImpl: account_register()");

        let account_repository = self.repository.lock().await;
        let result = account_repository.save(account_register_request.to_account().unwrap()).await;

        if result.is_ok() {
            AccountRegisterResponse::new(true)
        } else {
            eprintln!("계정 생성 중 에러 발생");
            AccountRegisterResponse::new(false)
        }
    }
}
