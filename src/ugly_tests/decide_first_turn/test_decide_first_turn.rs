
#[cfg(test)]
mod tests{
    use::rand::Rng;
    #[test]

    fn random_decide_first_turn()
    {
        #[derive(Debug)]
        enum Player {
            Player1,
            Player2,
        }
        let mut rng = rand::thread_rng();
        let random_number: u32 = rng.gen_range(0..=2);

        let selected_player = match random_number {
            0 => Player::Player1,
            1 => Player::Player2,
            _ => panic!("Unexpected random number"),
        };

        // 선택된 플레이어 출력
        println!("random number:{}",random_number);
        println!("Selected player: {:?}", selected_player);




    }
}