#[derive(Debug, Clone)] // Debug 및 Clone 트레이트를 자동으로 구현
pub struct Unitcard {
    pub card_id: i32,
    pub card_name: String,
    pub card_race: String,
    pub card_kinds: String,
    pub card_grade: i32,
    pub attack_point: i32,
    pub health_point: i32,
    pub activation_energy: i32,
    pub skill: Vec<i32>,
    pub passive: Vec<i32>,
}

#[derive(Debug)]
enum Cardgrade {
    general=1,
    uncommon,
    hero,
    legend,
    myth,
}

#[cfg(test)]
mod tests {
    // 유닛 카드 구조체 가져오기
    use super::{Unitcard};

    #[test]
    fn test_nomal_attack() {

        let unit_battlefield: Vec<Unitcard> = Vec::new();
        let mut filed_card:Vec<Unitcard> = Vec::new();
        filed_card.push(Unitcard {
            card_id: 0,
            card_name: String::from("듬직한_골렘"),
            card_race: String::from("트랜트족"),
            card_kinds: String::from("유닛"),
            card_grade: 1,
            attack_point: 2,
            health_point: 25,
            activation_energy: 0,
            skill: vec![1, 2, 3],    // 미구현
            passive: vec![4, 5, 6],  // 미구현
        });
        filed_card.push(Unitcard {
            card_id: 1,
            card_name: String::from("숙련된_병사"),
            card_race: String::from("휴먼족"),
            card_kinds: String::from("유닛"),
            card_grade: 1,
            attack_point: 5,
            health_point: 5,
            activation_energy: 1,
            skill: vec![7, 8, 9],       // 미구현
            passive: vec![10, 11, 12],  // 미구현
        });

        println!("Card[0] attack_point: {:?}", filed_card[0].attack_point);
        println!("Card[1] health_point: {:?}", filed_card[1].health_point);
        println!("\x1b[91mCard[0] 이 Card[1] 을 일반 공격합니다.\x1b[0m");

        // 0 번 카드 공격력으로, 1 번 카드 체력을 공격
        let health_point_after_nomal_attack = filed_card[1].health_point - filed_card[0].attack_point;

        // 1 번 카드 체력을 갱신
        filed_card[1].health_point = health_point_after_nomal_attack;

        // 공격 이후 1 번 카드 체력
        println!("Card[1] health_point_after_nomal_attack: {:?}", filed_card[1].health_point);
        }
    }