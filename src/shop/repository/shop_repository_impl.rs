use std::collections::{HashMap, HashSet};
use std::sync::Arc;
use async_trait::async_trait;
use lazy_static::lazy_static;
use tokio::sync::Mutex as AsyncMutex;
use diesel::query_dsl::methods::{FilterDsl, FindDsl};
use diesel::{Connection, MysqlConnection, QueryDsl, ExpressionMethods, RunQueryDsl, OptionalExtension, Insertable};
use diesel::result::Error;
use rand::Rng;
use crate::account_card::entity::account_card::account_cards::{account_id, card_count, card_id};
use crate::account_card::entity::account_card::account_cards::dsl::account_cards;
use crate::account_card::entity::account_card::AccountCard;
use crate::common::csv::csv_reader::csv_read;

use crate::common::env::env_detector::EnvDetector;
use crate::mysql_config::mysql_connection::MysqlDatabaseConnection;
use crate::shop::repository::shop_repository::ShopRepository;

pub struct ShopRepositoryImpl {
    mysql_database_connection: Arc<AsyncMutex<MysqlDatabaseConnection>>,
}

impl ShopRepositoryImpl {
    pub fn new(mysql_connection: Arc<AsyncMutex<MysqlDatabaseConnection>>) -> Self {
        ShopRepositoryImpl {
            mysql_database_connection: mysql_connection
        }
    }
    pub fn get_instance() -> Arc<AsyncMutex<ShopRepositoryImpl>> {
        lazy_static! {
            static ref INSTANCE: Arc<AsyncMutex<ShopRepositoryImpl>> =
                Arc::new(
                    AsyncMutex::new(
                        ShopRepositoryImpl::new(
                            MysqlDatabaseConnection::get_instance())));
        }
        INSTANCE.clone()
    }
}

#[async_trait]
impl ShopRepository for ShopRepositoryImpl {
    // async fn add_free_cards(&self, account_unique_id: i32) -> Result<Vec<i32>, Error> {
    //     use diesel::prelude::*;
    //
    //     println!("ShopRepositoryImpl: add_free_cards()");
    //
    //     let database_url = EnvDetector::get_mysql_url().expect("DATABASE_URL이 설정되어 있어야 합니다.");
    //     let mut connection = MysqlConnection::establish(&database_url)
    //         .expect("Failed to establish a new connection");
    //
    //     // 일단 모든 카드 다 로딩
    //     let where_clause = FilterDsl::filter(account_cards, account_id.eq(account_id));
    //     let found_cards = where_clause
    //         .select((account_id, card_id, card_count))
    //         .load::<AccountCard>(&mut connection)?;
    //
    //     println!("all cards found : {:?}", found_cards);
    //
    //     // 기존에 있던 정보 정리
    //     // TODO: 이렇게 하지 않고 update 하는 방식이 없는지 고민 필요
    //     diesel::delete(account_cards)
    //         .filter(account_id.eq(account_unique_id))
    //         .execute(&mut connection)
    //         .expect("Fail to remove original data");
    //
    //     // 계정 고유 ID 로 필터링
    //     let found_account_cards = found_cards.into_iter()
    //         .filter(|account_card| account_card.account_id == account_unique_id);
    //
    //     // 클라이언트의 카드 전체 리스트 확보
    //     let mut current_account_card_list = Vec::new();
    //     for card in found_account_cards {
    //         let card_count_index = card.card_count;
    //         for _ in 0..card_count_index {
    //             current_account_card_list.push(card.card_id);
    //         }
    //     }
    //
    //     println!("current account card list : {:?}", current_account_card_list);
    //     println!("and its length : {:?}", current_account_card_list.len());
    //
    //     // 무료 카드 발급
    //     let mut free_card_id_list = Self::get_randomly_chosen_card_id_list(10).unwrap();
    //
    //     current_account_card_list.extend_from_slice(&free_card_id_list);
    //     println!("final card list : {:?}", current_account_card_list);
    //
    //     let mut card_count_map = HashMap::new();
    //     let mut final_card_id_list_for_loop = current_account_card_list.clone();
    //
    //     // 카드 ID 로 카드 장수 알 수 있는 해시맵 형성
    //     for card_unique_id in final_card_id_list_for_loop {
    //         let card_counts = card_count_map.entry(card_unique_id).or_insert(0);
    //         *card_counts += 1;
    //     }
    //
    //     let mut final_card_id_list_to_remove_duplication = current_account_card_list.clone();
    //     let unique_card_id_set: HashSet<i32> = final_card_id_list_to_remove_duplication.into_iter().collect();
    //     let unique_card_id_list: Vec<i32> = unique_card_id_set.into_iter().collect();
    //
    //     for card_unique_id in unique_card_id_list {
    //         let card_count_index = card_count_map.get(&card_unique_id).expect("REASON").clone();
    //         let account_card = AccountCard::new(account_unique_id, card_unique_id, card_count_index).unwrap();
    //         diesel::replace_into(account_cards)
    //             .values((account_id.eq(account_card.account_id),
    //                             card_id.eq(account_card.card_id),
    //                             card_count.eq(account_card.card_count)))
    //             .execute(&mut connection)
    //             .expect("REASON");
    //     }
    //
    //     println!("Free cards saved successfully");
    //
    //     Ok(free_card_id_list)
    // }

    async fn get_randomly_chosen_card_id_list(&self, how_many_cards_to_get: i32, gacha_card_list: Vec<(i32,String)>) -> Result<Vec<i32>, Error> {
        let mut original_card_id_list = Vec::new();
        let mut randomly_chosen_card_id_list = Vec::new();
        //
        // let filename = "../../../resources/csv/every_card.csv";
        // match csv_read(filename) {
        //     Ok(csv_contents) => {
        //         for record in csv_contents {
        //             let card_number: i32 = record.get(6).unwrap().to_string().parse().expect("REASON");
        //             original_card_id_list.push(card_number);
        //         }
        //     }
        //     Err(err) => eprintln!("Error: {}", err)
        // }

        for card in gacha_card_list {
            original_card_id_list.push(card.0);
        }

        for _ in 0..how_many_cards_to_get {
            let random_index = rand::thread_rng().gen_range(0..original_card_id_list.len());
            randomly_chosen_card_id_list.push(original_card_id_list[random_index]);
        }

        println!("randomly_chosen_card_id_list : {:?}", randomly_chosen_card_id_list);

        Ok(randomly_chosen_card_id_list)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::test;

    #[test]
    async fn test_generate_random_card_list_from_file() {
    }
}