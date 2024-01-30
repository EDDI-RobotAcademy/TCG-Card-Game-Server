// 플레이어는 자신의 턴에 필드에 자신의 핸드에 있던 카드를 필드에 내려놓는 행위를 할 수 있습니다.

use std::collections::BTreeMap;
use rand::Rng;
use crate::ugly_tests::put_card_on_the_field::entity::sample_card::SampleCard;

pub(crate) fn sample_card() -> SampleCard {
    let random_race = rand::thread_rng().gen_range(1..=5);
    let random_grade = rand::thread_rng().gen_range(1..=5);
    let random_type = rand::thread_rng().gen_range(1..=5);
    let random_activation_energy = rand::thread_rng().gen_range(1..=3);
    let random_attack_point = rand::thread_rng().gen_range(5..=10);
    let random_health_point = rand::thread_rng().gen_range(5..=10);
    SampleCard::new(random_race, random_grade,
                    random_type, random_activation_energy,
                    random_attack_point, random_health_point)
}

pub(crate) fn random_card_map(card_id_list: Vec<i32>) -> BTreeMap<i32, SampleCard> {
    let mut btreemap = BTreeMap::new();
    for id in card_id_list {
        let card = sample_card();
        btreemap.insert(id, card);
    }
    btreemap
}
pub(crate) fn check_usability(card_id: i32, map: &BTreeMap<i32, SampleCard>) {
    let mut card = map.get(&card_id).unwrap();
    if card.card_grade() == 5 {
        println!("신화 카드는 첫 턴에 사용 불가능!")
    } else if card.card_type() == 4 {
        println!("서포트 카드는 한 턴에 한 장만 사용 가능")
    } else {
        println!("카드를 사용합니다!");
        // println!("{:?}", card);
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use tokio::test;

    #[test]
    async fn test_put_card_on_the_field() {
        let card_id_list = [10, 20, 30, 40, 50].to_vec();
        let card_id_map = random_card_map(card_id_list.clone());

        for id in card_id_list {
            check_usability(id, &card_id_map);
        }
    }
}