use rand::Rng;
use crate::ugly_tests::tutorial::player::entity::player::Player;

// 실제 배틀에서 감지해야 하는 모든 status 에 대해 시뮬레이션을 진행합니다.
fn generate_random_deck() -> Vec<i32> {
    let mut random_card_list = Vec::new();
    for _ in 0..40 {
        random_card_list.push(rand::thread_rng().gen_range(1..100));
    }
    println!("{}장의 랜덤 덱 생성 완료!", random_card_list.len());
    random_card_list
}
#[cfg(test)]
mod tests {
    use super::*;
    use tokio::test;
    use crate::ugly_tests::tutorial::player::entity::player::Player;

    #[test]
    async fn generate_player() {

        println!("0. 랜덤 덱 생성");

        let deck1 = generate_random_deck();
        let deck2 = generate_random_deck();

        println!("1. 플레이어 생성");

        let mut player1 = Player::new();
        let mut player2 = Player::new();

        println!("Player 1 : {:?}", player1);
        println!("Player 2 : {:?}", player2);

        println!("2. 플레이어 덱 추가");

        player1.append_cards_to_deck(deck1);
        player2.append_cards_to_deck(deck2);

        println!("Player 1 : {:?}", player1);
        println!("Player 2 : {:?}", player2);

        println!("3. 최초로 각 플레이어 5장씩 드로우");
        player1.draw_cards_from_deck(5);
        player2.draw_cards_from_deck(5);
        println!("Player 1 Hand: {:?}", player1.get_hand());
        println!("Player 1 Remaining Deck: {:?}", player1.get_deck());
        println!("Player 2 Hand: {:?}", player2.get_hand());
        println!("Player 2 Remaining Deck: {:?}", player2.get_deck());

        // 일단 멀리건 생략
        println!("4. 한 명 파괴 시까지 반복");
        while player2.get_life_point() > 0 {
            println!("Player 1 : 턴 시작");
            player1.add_turn_number(1);
            player1.add_field_energy(2);
            player1.draw_cards_from_deck(1);
            println!("Player 1 Turn Number: {:?} ", player1.get_turn_number());
            println!("Player 1 Field Energy: {:?}", player1.get_field_energy());
            println!("Player 1 Hand: {:?}", player1.get_hand());
            println!("Player 1 Remaining Deck: {:?}", player1.get_deck());

            println!("Player 1 : 카드 1 장을 필드에 제시");
            player1.use_card_from_hand(player1.get_hand()[rand::thread_rng().gen_range(0..5)]);
            println!("Player 1 Hand: {:?}", player1.get_hand());
            println!("Player 1 Field: {:?}", player1.get_field());

            println!("Player 1 : 카드에 에너지를 붙여 상대에게 랜덤 데미지를 가함");

            player1.use_field_energy_to_card(1);
            player2.set_life_point(rand::thread_rng().gen_range(-10..0));

            println!("Player 1 Field Energy: {:?}", player1.get_field_energy());
            println!("Player 2 LP: {:?}", player2.get_life_point());
        }
    }
}