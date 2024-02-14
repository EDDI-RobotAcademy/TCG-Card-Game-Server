use crate::game_main_character::entity::health_point::HealthPoint;

#[derive(Debug)]
pub struct GameMainCharacter {
    health_point: HealthPoint
}

impl GameMainCharacter {
    pub fn new(initial_health: i32) -> GameMainCharacter {
        GameMainCharacter {
            health_point: HealthPoint::new(initial_health),
        }
    }

    pub fn get_health_point(&self) -> i32 {
        self.health_point.get_health()
    }

    pub fn decrease_health_point(&mut self, damage: i32) { self.health_point.lose_health(damage) }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_game_main_character() {
        let main_character = GameMainCharacter::new(150);
        println!("GameMainCharacter HealthPoint: {}", main_character.get_health_point());
        assert_eq!(main_character.get_health_point(), 150);

        let main_character = GameMainCharacter::new(200);
        println!("GameMainCharacter HealthPoint: {}", main_character.get_health_point());
        assert_eq!(main_character.get_health_point(), 200);
    }
}
