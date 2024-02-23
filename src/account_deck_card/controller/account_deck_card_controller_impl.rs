use std::sync::Arc;
use async_trait::async_trait;
use lazy_static::lazy_static;
use tokio::sync::Mutex as AsyncMutex;
use crate::account_deck::service::account_deck_service::AccountDeckService;
use crate::account_deck::service::account_deck_service_impl::AccountDeckServiceImpl;

use crate::account_deck_card::controller::account_deck_card_controller::AccountDeckCardController;
use crate::account_deck_card::controller::request_form::account_deck_card_list_request_form::AccountDeckCardListRequestFrom;
use crate::account_deck_card::controller::request_form::account_deck_card_modify_request_form::AccountDeckCardModifyRequestForm;
use crate::account_deck_card::controller::request_form::account_deck_configuration_request_form::AccountDeckConfigurationRequestForm;
use crate::account_deck_card::controller::response_form::account_deck_card_list_response_form::AccountDeckCardListResponseForm;
use crate::account_deck_card::controller::response_form::account_deck_card_modify_response_form::AccountDeckCardModifyResponseForm;
use crate::account_deck_card::controller::response_form::account_deck_configuration_response_form::AccountDeckConfigurationResponseForm;

use crate::account_deck_card::service::account_deck_card_service::AccountDeckCardService;
use crate::account_deck_card::service::account_deck_card_service_impl::AccountDeckCardServiceImpl;
use crate::deck_configuration_validator::service::deck_configuration_validator_service::DeckConfigurationValidatorService;
use crate::deck_configuration_validator::service::deck_configuration_validator_service_impl::DeckConfigurationValidatorServiceImpl;

pub struct AccountDeckCardControllerImpl {
    account_deck_card_service: Arc<AsyncMutex<AccountDeckCardServiceImpl>>,
    deck_configuration_validator_service: Arc<AsyncMutex<DeckConfigurationValidatorServiceImpl>>,
    account_deck_service: Arc<AsyncMutex<AccountDeckServiceImpl>>,
}

impl AccountDeckCardControllerImpl {
    pub fn new(account_deck_card_service: Arc<AsyncMutex<AccountDeckCardServiceImpl>>,
               deck_configuration_validator_service: Arc<AsyncMutex<DeckConfigurationValidatorServiceImpl>>,
               account_deck_service: Arc<AsyncMutex<AccountDeckServiceImpl>>, ) -> Self {
        AccountDeckCardControllerImpl {
            account_deck_card_service,
            deck_configuration_validator_service,
            account_deck_service,
        }
    }
    pub fn get_instance() -> Arc<AsyncMutex<AccountDeckCardControllerImpl>> {
        lazy_static! {
            static ref INSTANCE: Arc<AsyncMutex<AccountDeckCardControllerImpl>> =
                Arc::new(
                    AsyncMutex::new(
                        AccountDeckCardControllerImpl::new(
                            AccountDeckCardServiceImpl::get_instance(),
                            DeckConfigurationValidatorServiceImpl::get_instance(),
                            AccountDeckServiceImpl::get_instance())));
        }
        INSTANCE.clone()
    }
}

#[async_trait]
impl AccountDeckCardController for AccountDeckCardControllerImpl {
    async fn deck_configuration_register(
        &self, account_deck_configuration_request_form: AccountDeckConfigurationRequestForm) -> AccountDeckConfigurationResponseForm {
        println!("AccountDeckCardControllerImpl: deck_configuration_register()");

        let account_deck_service_guard = self.account_deck_service.lock().await;
        let account_deck_owner_verification = account_deck_service_guard.account_deck_owner_verification(
            account_deck_configuration_request_form.account_session_id(),
            account_deck_configuration_request_form.deck_id_form()).await;
        if account_deck_owner_verification == false {
            return AccountDeckConfigurationResponseForm::new(false, "not your deck".to_string());
        }

        let deck_configuration_validator_service_guard =
            self.deck_configuration_validator_service.lock().await;

        let validation_card_owner = deck_configuration_validator_service_guard
            .do_you_have_this_card(
                account_deck_configuration_request_form.card_id_list_form(),
                account_deck_configuration_request_form.account_session_id()).await;
        if validation_card_owner == false {
            return AccountDeckConfigurationResponseForm::new(false, "not your card".to_string())
        }

        let validation_result = deck_configuration_validator_service_guard
            .validate_deck(&account_deck_configuration_request_form.card_id_list_form()).await;

        return match validation_result {
            Ok(()) => {
                let account_deck_card_service_guard =
                    self.account_deck_card_service.lock().await;

                let response = account_deck_card_service_guard
                    .deck_configuration_register(
                        account_deck_configuration_request_form.to_account_deck_configuration_request()).await;

                response.to_account_deck_configuration_response_form()
            }
            Err(error_message) => {
                AccountDeckConfigurationResponseForm::new(false, error_message)
            }
        }
    }
    async fn deck_card_list(&self, account_deck_card_list_request_from: AccountDeckCardListRequestFrom) -> AccountDeckCardListResponseForm {
        println!("AccountDeckCardControllerImpl: deck_card_list()");

        let account_deck_card_service_guard
            = self.account_deck_card_service.lock().await;

        let response = account_deck_card_service_guard
            .deck_card_list(account_deck_card_list_request_from.to_account_deck_card_list_request()).await;

        response.to_account_deck_card_list_response_form()
    }
    async fn deck_card_modify(
        &self, account_deck_card_modify_request_form: AccountDeckCardModifyRequestForm) -> AccountDeckCardModifyResponseForm {
        println!("AccountDeckCardControllerImpl: deck_card_modify()");

        let account_deck_service_guard = self.account_deck_service.lock().await;
        let account_deck_owner_verification = account_deck_service_guard.account_deck_owner_verification(
            account_deck_card_modify_request_form.account_session_id(),
            account_deck_card_modify_request_form.deck_id_form()).await;
        if account_deck_owner_verification == false {
            return AccountDeckCardModifyResponseForm::new(false);
        }

        let deck_configuration_validator_service_guard =
            self.deck_configuration_validator_service.lock().await;

        let validation_card_owner = deck_configuration_validator_service_guard
            .do_you_have_this_card(
                account_deck_card_modify_request_form.card_id_list_form(),
                account_deck_card_modify_request_form.account_session_id()).await;
        if validation_card_owner == false {
            return AccountDeckCardModifyResponseForm::new(false)
        }

        let validation_result = deck_configuration_validator_service_guard
            .validate_deck(&account_deck_card_modify_request_form.card_id_list_form()).await;

        return match validation_result {
            Ok(()) => {
                let account_deck_card_service_guard =
                    self.account_deck_card_service.lock().await;
                let _ = account_deck_card_service_guard.deck_card_delete(account_deck_card_modify_request_form.deck_id_form()).await;

                let response = account_deck_card_service_guard
                    .deck_configuration_register(
                        account_deck_card_modify_request_form.to_account_deck_configuration_request()).await;

                AccountDeckCardModifyResponseForm::new(response.get_is_success())
            }
            Err(error_message) => {
                AccountDeckCardModifyResponseForm::new(false)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::test;

    #[test]
    async fn test_deck_configuration_register_in_controller() {

        let card_list = vec![
            19, 8, 8, 8, 9, 9, 25, 25, 25, 27, 27, 27, 151, 20, 20, 20, 2, 2, 2,
            26, 26, 26, 30, 31, 31, 31, 32, 32, 32, 33, 33, 35, 35, 36, 36, 93, 93, 93, 130, 130
        ];

        let deck_configuration_request_form
            = AccountDeckConfigurationRequestForm::new(23, card_list, "qwer".to_string());

        let account_deck_card_controller = AccountDeckCardControllerImpl::get_instance();
        let account_deck_card_controller_guard = account_deck_card_controller.lock().await;

        let result = account_deck_card_controller_guard
            .deck_configuration_register(deck_configuration_request_form).await;

        println!("is_saved: {:?}", result.get_is_success());
    }
}