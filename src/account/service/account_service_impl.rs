use std::sync::Arc;
use async_trait::async_trait;
use bcrypt::{hash, verify};
use diesel::dsl::not;
use lazy_static::lazy_static;
use tokio::sync::Mutex as AsyncMutex;
use uuid::Uuid;

use crate::account::repository::account_repository::AccountRepository;
use crate::account::repository::account_repository_impl::AccountRepositoryImpl;
use crate::account_point::repository::account_point_repository::AccountPointRepository;
use crate::account_point::repository::account_point_repository_impl::AccountPointRepositoryImpl;

use crate::account::service::account_service::AccountService;

use crate::account::service::request::account_session_logout_request::AccountSessionLogoutRequest;
use crate::account::service::request::account_session_login_request::AccountSessionLoginRequest;

use crate::account::service::request::account_register_request::AccountRegisterRequest;
use crate::account::service::request::account_logout_request::AccountLogoutRequest;
use crate::account::service::request::account_delete_request::AccountDeleteRequest;
use crate::account::service::request::account_modify_request::AccountModifyRequest;
use crate::account::service::request::account_login_request::AccountLoginRequest;

use crate::account::service::response::account_register_response::AccountRegisterResponse;
use crate::account::service::response::account_logout_response::AccountLogoutResponse;
use crate::account::service::response::account_delete_response::AccountDeleteResponse;
use crate::account::service::response::account_modify_response::AccountModifyResponse;
use crate::account::service::response::account_login_response::AccountLoginResponse;
use crate::account_card::repository::account_card_repository::AccountCardRepository;
use crate::account_card::repository::account_card_repository_impl::AccountCardRepositoryImpl;
use crate::account_deck::repository::account_deck_repository::AccountDeckRepository;
use crate::account_deck::repository::account_deck_repository_impl::AccountDeckRepositoryImpl;
use crate::account_deck_card::repository::account_deck_card_repository::AccountDeckCardRepository;
use crate::account_deck_card::repository::account_deck_card_repository_impl::AccountDeckCardRepositoryImpl;

use crate::redis::repository::redis_in_memory_repository::RedisInMemoryRepository;

use crate::redis::repository::redis_in_memory_repository_impl::RedisInMemoryRepositoryImpl;


pub struct AccountServiceImpl {
    repository: Arc<AsyncMutex<AccountRepositoryImpl>>,
    account_point_repository: Arc<AsyncMutex<AccountPointRepositoryImpl>>,
    redis_in_memory_repository: Arc<AsyncMutex<RedisInMemoryRepositoryImpl>>,
    account_card_repository: Arc<AsyncMutex<AccountCardRepositoryImpl>>,
    account_deck_repository: Arc<AsyncMutex<AccountDeckRepositoryImpl>>,
    account_deck_card_repository: Arc<AsyncMutex<AccountDeckCardRepositoryImpl>>
}

impl AccountServiceImpl {
    pub fn new(repository: Arc<AsyncMutex<AccountRepositoryImpl>>,
               account_point_repository: Arc<AsyncMutex<AccountPointRepositoryImpl>>,
               redis_in_memory_repository: Arc<AsyncMutex<RedisInMemoryRepositoryImpl>>,
               account_card_repository: Arc<AsyncMutex<AccountCardRepositoryImpl>>,
               account_deck_repository: Arc<AsyncMutex<AccountDeckRepositoryImpl>>,
               account_deck_card_repository: Arc<AsyncMutex<AccountDeckCardRepositoryImpl>>) -> Self {

        AccountServiceImpl {
            repository,
            account_point_repository,
            redis_in_memory_repository,
            account_card_repository,
            account_deck_repository,
            account_deck_card_repository,
        }
    }

    pub fn get_instance() -> Arc<AsyncMutex<AccountServiceImpl>> {
        lazy_static! {
            static ref INSTANCE: Arc<AsyncMutex<AccountServiceImpl>> =
                Arc::new(
                    AsyncMutex::new(
                        AccountServiceImpl::new(
                            AccountRepositoryImpl::get_instance(),
                            AccountPointRepositoryImpl::get_instance(),
                            RedisInMemoryRepositoryImpl::get_instance(),
                            AccountCardRepositoryImpl::get_instance(),
                            AccountDeckRepositoryImpl::get_instance(),
                            AccountDeckCardRepositoryImpl::get_instance())));
        }
        INSTANCE.clone()
    }
}

#[async_trait]
impl AccountService for AccountServiceImpl {
    async fn account_register(&self, account_register_request: AccountRegisterRequest) -> AccountRegisterResponse {
        println!("AccountServiceImpl: account_register()");

        let account_repository = self.repository.lock().await;
        let account_point_repository = self.account_point_repository.lock().await;
        let account = account_register_request.to_account().unwrap();
        // 중복 확인 작업
        if account_repository.find_by_user_id(account.user_id()).await.unwrap().is_some() {
            // 중복 아이디 존재하는 경우
            return AccountRegisterResponse::new(false)
        }
        // 중복 아이디 존재하지 않는 경우 계정 저장
        let result_account = account_repository.save(account_register_request.to_account().unwrap()).await;

        // 저장된 계정의 id 를 가지고, account_point 에 재화 관련 계정정보 생성
        let found_account = account_repository.find_by_user_id(account.user_id()).await.unwrap();
        let found_account_id = found_account.unwrap().id;
        let set_account_point_id = account_point_repository.set_account_point(found_account_id,100).await;
        let result_account_point = account_point_repository.save_account_points(set_account_point_id).await;

        if result_account.is_ok() && result_account_point.is_ok() {
            return AccountRegisterResponse::new(true)
        }
        return AccountRegisterResponse::new(false)
    }

    async fn account_login(&self, account_login_request: AccountLoginRequest) -> AccountLoginResponse {
        println!("AccountServiceImpl: account_login()");

        let account_repository = self.repository.lock().await;
        let account = account_login_request.to_account().unwrap();
        if let Some(found_account) = account_repository.find_by_user_id(account.user_id()).await.unwrap() {
            // 비밀번호 매칭 확인
            if verify(&account_login_request.password(), &found_account.password()).unwrap() {
                // 로그인 성공
                let redis_token = Uuid::new_v4();
                let mut redis_repository_guard = self.redis_in_memory_repository.lock().await;
                redis_repository_guard.set_with_expired_time(&*redis_token.to_string(), &found_account.id.to_string(), Some(3600)).await;

                return AccountLoginResponse::new(redis_token.to_string());
            } else {
                // 비밀번호 불일치 - 로그인 실패
                eprintln!("Password mismatch for user_id: {}", account.user_id());
                eprintln!("Password mismatch for password: {}", account.password());
                eprintln!("login_request password: {}", account_login_request.password());

                // Output hashed password stored in the database
                println!("Stored hashed password: {}", found_account.password());
                println!("Generated hashed password: {}", hash(&account_login_request.password(), 12).unwrap());
            }
        } else {
            // 계정이 없음 - 로그인 실패
            eprintln!("Account not found for user_id: {}", account.user_id());
        }

        // 로그인 실패 시 기본 응답 반환
        AccountLoginResponse::new("".to_string())
    }

    async fn account_logout(&self, account_logout_request: AccountLogoutRequest) -> AccountLogoutResponse {
        println!("AccountServiceImpl: account_logout()");

        let mut redis_repository_guard = self.redis_in_memory_repository.lock().await;
        let account_session_id = redis_repository_guard.get(account_logout_request.get_session_id()).await;

        if let Some(id) = account_session_id{
            redis_repository_guard.del(account_logout_request.get_session_id()).await;
            return AccountLogoutResponse::new(true)
        }
        return AccountLogoutResponse::new(false)
    }

    // TODO: Session Domain 혹은 Authentication Domain 을 별도로 구성하는 것이 더 좋을 것이다.
    async fn account_session_login(&self, account_session_login_request: AccountSessionLoginRequest) -> AccountLoginResponse {
        println!("AccountServiceImpl: account_session_login()");

        let mut redis_repository_gaurd = self.redis_in_memory_repository.lock().await;
        let account_unique_id = redis_repository_gaurd.get(account_session_login_request.get_session_id()).await;

        if let Some(id) = account_unique_id {
            AccountLoginResponse::new(account_session_login_request.get_session_id().to_string())
        } else {
            AccountLoginResponse::new("".to_string())
        }
    }

    async fn account_session_logout(&self, account_session_logout_request: AccountSessionLogoutRequest) -> AccountLogoutResponse {
        println!("AccountServiceImpl: account_session_logout()");

        let mut redis_repository_gaurd = self.redis_in_memory_repository.lock().await;
        let account_unique_id = redis_repository_gaurd.get(account_session_logout_request.get_session_id()).await;

        if let Some(id) = account_unique_id {
            redis_repository_gaurd.del(account_session_logout_request.get_session_id()).await;
            return AccountLogoutResponse::new(true)
        }
        return AccountLogoutResponse::new(false)
    }

    async fn account_delete(&self, account_delete_request: AccountDeleteRequest) -> AccountDeleteResponse {
        println!("AccountServiceImpl: account_delete()");

        // TODO: 지금 너무 위험한 방식입니다. 세션 정보 없이 사용자 ID, PW 만으로 지웠다가는 다른 사람 것을 통채로 밀어버릴 수 있습니다.
        // TODO: 세션 정보를 받아서 지금 로그인 되어있는 사용자의 고유 ID 와 user_id 로 찾은 계정의 고유값이 일치하는지도 확인을 추가로 해야합니다.
        let account_repository_guard = self.repository.lock().await;
        let account = account_delete_request.to_account().unwrap();

        if let Some(found_account) = account_repository_guard.find_by_user_id(account.user_id()).await.unwrap() {
            if verify(&account_delete_request.password(), &found_account.password()).unwrap() {
                let account_unique_id = found_account.id;

                let account_card_repository_guard = self.account_card_repository.lock().await;
                let _ = account_card_repository_guard.delete_all_account_cards(account_unique_id).await;
                drop(account_card_repository_guard);

                let account_deck_repository_guard = self.account_deck_repository.lock().await;
                let account_deck_id_list = account_deck_repository_guard.get_account_deck_id_list(account_unique_id).await.unwrap();

                let account_deck_card_repository_guard = self.account_deck_card_repository.lock().await;
                for deck_id in account_deck_id_list {
                    let _ = account_deck_card_repository_guard.delete_deck_cards(deck_id).await;
                }
                drop(account_deck_card_repository_guard);

                let _ = account_deck_repository_guard.delete_all_account_decks(account_unique_id).await;
                drop(account_deck_repository_guard);

                let account_point_repository_guard = self.account_point_repository.lock().await;
                let _ = account_point_repository_guard.delete_account_points(account_unique_id).await;
                drop(account_point_repository_guard);

                // TODO: redis 도 정리해야 함

                let _ = account_repository_guard.delete(found_account).await;
                drop(account_repository_guard);

                // TODO: redis 를 정리했기 때문에 UI 쪽 sessionInfo 를 지울 수 있도록 "" 도 함께 돌려줘야 함
                return AccountDeleteResponse::new(true)
            }
        }

        println!("Account is not found.");
        AccountDeleteResponse::new(false)
    }

    async fn account_modify(&self, account_modify_request: AccountModifyRequest) -> AccountModifyResponse {
        println!("AccountServiceImpl: account_modify()");

        let account_repository = self.repository.lock().await;
        let account = account_modify_request.to_account().unwrap();
        // Database accounts tables 에서 계정 찾기
        if let Some(found_account) = account_repository.find_by_user_id(account.user_id()).await.unwrap() {
            // 비밀번호 매칭 확인
            if verify(&account_modify_request.password(), &found_account.password()).unwrap() {
                let result = account_repository.update(account_modify_request.to_account().unwrap(), account_modify_request.new_password()).await;
                if result.is_ok() {
                    return AccountModifyResponse::new(true)
                }
                return AccountModifyResponse::new(false)
            }
            eprintln!("The password does not match.");
            return AccountModifyResponse::new(false)
        }
        // 계정이 없음 - 로그인 실패
        eprintln!("Account not found for user_id: {}", account.user_id());
        return AccountModifyResponse::new(false)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::test;

    #[test]
    async fn test_account_delete() {
        let account_service_mutex = AccountServiceImpl::get_instance();
        let account_service = account_service_mutex.lock().await;

        let account_delete_request = AccountDeleteRequest::new("test", "test".to_string());

        account_service.account_delete(account_delete_request).await;
    }
}