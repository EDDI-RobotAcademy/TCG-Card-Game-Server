use async_trait::async_trait;
use crate::shop::service::request::data_to_display_in_shop_request::DataToDisplayInShopRequest;
use crate::shop::service::response::data_to_display_in_shop_response::DataToDisplayInShopResponse;

#[async_trait]
pub trait ShopService {
    async fn data_to_display_in_shop(&self, data_to_display_in_shop_request: DataToDisplayInShopRequest) -> DataToDisplayInShopResponse ;

}