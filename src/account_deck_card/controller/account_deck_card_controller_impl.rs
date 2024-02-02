use std::sync::Arc;
use async_trait::async_trait;
use lazy_static::lazy_static;
use tokio::sync::Mutex as AsyncMutex;
use crate::account_deck_card::controller::account_deck_card_controller::AccountDeckCardController;
use crate::account_deck_card::controller::request::account_deck_card_list_request::AccountDeckCardListRequest;
use crate::account_deck_card::controller::request::account_deck_configuration_request::AccountDeckConfigurationRequest;
use crate::account_deck_card::controller::response::account_deck_card_list_response::AccountDeckCardListResponse;
use crate::account_deck_card::controller::response::account_deck_configuration_response::AccountDeckConfigurationResponse;
use crate::account_deck_card::service::account_deck_card_service::AccountDeckCardService;
use crate::account_deck_card::service::account_deck_card_service_impl::AccountDeckCardServiceImpl;
use crate::deck_configuration_validator::service::deck_configuration_validator_service::DeckConfigurationValidatorService;
use crate::deck_configuration_validator::service::deck_configuration_validator_service_impl::DeckConfigurationValidatorServiceImpl;

pub struct AccountDeckCardControllerImpl {
    account_deck_card_service: Arc<AsyncMutex<AccountDeckCardServiceImpl>>,
    deck_configuration_validator_service: Arc<AsyncMutex<DeckConfigurationValidatorServiceImpl>>
}

impl AccountDeckCardControllerImpl {
    pub fn new(account_deck_card_service: Arc<AsyncMutex<AccountDeckCardServiceImpl>>,
               deck_configuration_validator_service: Arc<AsyncMutex<DeckConfigurationValidatorServiceImpl>>) -> Self {
        AccountDeckCardControllerImpl {
            account_deck_card_service,
            deck_configuration_validator_service
        }
    }
    pub fn get_instance() -> Arc<AsyncMutex<AccountDeckCardControllerImpl>> {
        lazy_static! {
            static ref INSTANCE: Arc<AsyncMutex<AccountDeckCardControllerImpl>> =
                Arc::new(
                    AsyncMutex::new(
                        AccountDeckCardControllerImpl::new(
                            AccountDeckCardServiceImpl::get_instance(),
                            DeckConfigurationValidatorServiceImpl::get_instance())));
        }
        INSTANCE.clone()
    }
}

#[async_trait]
impl AccountDeckCardController for AccountDeckCardControllerImpl {
    async fn deck_configuration_register(
        &self, deck_configuration_request: AccountDeckConfigurationRequest) -> AccountDeckConfigurationResponse {

        let deck_configuration_validator_service_guard
            = self.deck_configuration_validator_service.lock().await;

        let validation_result
            = deck_configuration_validator_service_guard.validate_deck(deck_configuration_request.card_id_list_of_deck()).await;

        return match validation_result {
            Ok(()) => {
                let account_deck_card_service_guard
                    = self.account_deck_card_service.lock().await;
                let response
                    = account_deck_card_service_guard.deck_configuration_register(deck_configuration_request).await;
                response
            }
            Err(error_message) => {
                AccountDeckConfigurationResponse::new(false, error_message)
            }
        }
    }
    async fn deck_card_list(&self, deck_card_list_request: AccountDeckCardListRequest) -> AccountDeckCardListResponse {
        let account_deck_card_service_guard
            = self.account_deck_card_service.lock().await;

        let response = account_deck_card_service_guard.deck_card_list(deck_card_list_request).await;
        response
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
            26, 26, 26, 30, 31, 31, 31, 32, 32, 32, 33, 33, 35, 35, 36, 36, 93, 93, 93, 100, 100
        ];

        let deck_configuration_request = AccountDeckConfigurationRequest::new(777, card_list);

        // let deck_card_service = AccountDeckCardServiceImpl::get_instance();
        // let deck_card_service_guard = deck_card_service.lock().await;
        //
        // let result = deck_card_service_guard.deck_configuration_register(deck_configuration_request.clone()).await;

        let account_deck_card_controller = AccountDeckCardControllerImpl::get_instance();
        let account_deck_card_controller_guard = account_deck_card_controller.lock().await;

        let result = account_deck_card_controller_guard.deck_configuration_register(deck_configuration_request).await;

        println!("is_saved: {:?}", result.get_is_success());
        println!("message: {:?}", result.get_message());
    }
}