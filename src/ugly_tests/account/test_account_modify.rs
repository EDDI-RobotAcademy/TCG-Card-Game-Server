use crate::account::service::account_service_impl::AccountServiceImpl;
use crate::account::service::request::account_login_request::AccountLoginRequest;
use crate::account::service::request::account_register_request::AccountRegisterRequest;
use crate::account::service::request::account_modify_request::AccountModifyRequest;

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::test;
    use crate::account::service::account_service::AccountService;

    #[tokio::test]
    async fn test_account_modify_in_account_service() {
        let mut account_service_mutex = AccountServiceImpl::get_instance();
        let mut account_service_gaurd = account_service_mutex.lock().await;

        let test_id = "test_account_modify2";
        let test_pw = "test_account_modify2";
        let test_new_pw = "test_account_new_password2";
        let string_test_pw = test_pw.to_string();
        let string_test_new_pw = test_new_pw.to_string();

        //test_account 저장
        // let test_account = AccountRegisterRequest::new(test_id, string_test_pw);
        // account_service_gaurd.account_register(test_account).await;

        //test_account 계정정보 수정
        // let test_account = AccountModifyRequest::new(test_id,string_test_pw ,string_test_new_pw);
        // account_service_gaurd.account_modify(test_account).await;

        //test_account 변경전 password 으로 로그인 (fail 시 test ok)
        // let test_account = AccountLoginRequest::new(test_id, string_test_pw);
        // account_service_gaurd.account_login(test_account).await;

        //test_account 변경후 password 으로 로그인 (success 시 test ok)
        let test_account = AccountLoginRequest::new(test_id, string_test_new_pw);
        account_service_gaurd.account_login(test_account).await;
    }
}
