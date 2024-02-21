use async_trait::async_trait;
use crate::shop::controller::request_form::execute_free_gacha_request_form::ExecuteFreeGachaRequestForm;
use crate::shop::controller::request_form::execute_shop_gacha_request_form::ExecuteShopGachaRequestForm;
use crate::shop::controller::request_form::show_me_the_money_request::ShowMeTheMoneyRequest;
use crate::shop::controller::response_form::execute_free_gacha_response_form::ExecuteFreeGachaResponseForm;
use crate::shop::controller::response_form::execute_shop_gacha_response_form::ExecuteShopGachaResponseForm;
use crate::shop::controller::response_form::show_me_the_money_response::ShowMeTheMoneyResponse;

#[async_trait]
pub trait  ShopController {
    async fn execute_shop_gacha(&self, execute_shop_gacha_request_form: ExecuteShopGachaRequestForm) -> ExecuteShopGachaResponseForm;
    async fn execute_free_gacha(&self, execute_free_gacha_request_form: ExecuteFreeGachaRequestForm) -> ExecuteFreeGachaResponseForm;
    // async fn event_distribute_cards(&self, event_distribute_cards_request_form: EventDistributeCardsRequestForm) -> EventDistributeCardsResponseForm;
    async fn show_me_the_money(&self, show_me_the_money_request: ShowMeTheMoneyRequest) -> ShowMeTheMoneyResponse;

}
