use crate::account::service::response::account_register_response::AccountRegisterResponse;
use serde::{Deserialize, Serialize};
use crate::account::service::response::account_login_response::AccountLoginResponse;
use crate::account::service::response::account_logout_response::AccountLogoutResponse;
use crate::account::service::response::account_modify_response::AccountModifyResponse;
use crate::account::service::response::account_delete_response::AccountDeleteResponse;
use crate::account_deck::service::response::account_deck_list_response::AccountDeckListResponse;
use crate::account_deck::service::response::account_deck_modify_response::AccountDeckModifyResponse;
use crate::account_deck::service::response::account_deck_register_response::AccountDeckRegisterResponse;
use crate::battle_ready_monitor::service::response::battle_ready_response::BattleReadyResponse;
use crate::battle_room::service::response::battle_match_response::BattleMatchResponse;
use crate::client_program::service::response::client_program_exit_response::ClientProgramExitResponse;
use crate::deck_card::service::response::deck_card_list_response::DeckCardListResponse;
use crate::deck_card::service::response::deck_configuration_response::DeckConfigurationResponse;
use crate::shop::service::response::free_card_response::FreeCardResponse;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ResponseType {
    ACCOUNT_REGISTER(AccountRegisterResponse),
    ACCOUNT_LOGIN(AccountLoginResponse),
    ACCOUNT_SESSION_LOGIN(AccountLoginResponse),
    ACCOUNT_LOGOUT(AccountLogoutResponse),
    ACCOUNT_MODIFY(AccountModifyResponse),
    ACCOUNT_DELETE(AccountDeleteResponse),

    ACCOUNT_DECK_REGISTER(AccountDeckRegisterResponse),
    ACCOUNT_DECK_LIST(AccountDeckListResponse),
    ACCOUNT_DECK_MODIFY(AccountDeckModifyResponse),

    DECK_CARD_CONFIGURATION(DeckConfigurationResponse),
    DECK_CARD_LIST(DeckCardListResponse),

    FREE_CARD_BUY(FreeCardResponse),

    BATTLE_DECK_LIST(AccountDeckListResponse),
    BATTLE_MATCH(BattleMatchResponse),
    BATTLE_READY(BattleReadyResponse),
    BATTLE_DECK_CARD_LIST(DeckCardListResponse),

    PROGRAM_EXIT(ClientProgramExitResponse),
}
