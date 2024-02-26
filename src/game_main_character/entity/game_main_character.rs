use crate::game_main_character::entity::health_point::HealthPoint;
use crate::game_main_character::entity::status_main_character::StatusMainCharacterEnum;
use crate::game_main_character::entity::status_main_character::StatusMainCharacterEnum::Survival;


#[derive(Debug, Clone)]
pub struct GameMainCharacter {
    health_point: HealthPoint,
    status: StatusMainCharacterEnum,
}

impl GameMainCharacter {
    pub fn new(initial_health: i32) -> GameMainCharacter {
        GameMainCharacter {
            health_point: HealthPoint::new(initial_health),
            status: Survival,
        }
    }

    pub fn get_health_point(&self) -> i32 {
        self.health_point.get_health()
    }

    pub fn get_status(&self) -> &StatusMainCharacterEnum { &self.status }

    pub fn set_status(&mut self, status_update: StatusMainCharacterEnum) {
        self.status = status_update
    }

    pub fn decrease_health_point(&mut self, damage: i32) { self.health_point.lose_health(damage) }
}

#[cfg(test)]
mod tests {
    use crate::game_main_character::entity::status_main_character::StatusMainCharacterEnum::Death;
    use super::*;

    #[test]
    fn test_game_main_character() {
        let mut main_character = GameMainCharacter::new(150);
        println!("GameMainCharacter HealthPoint: {}", main_character.get_health_point());
        println!("GameMainCharacter status: {:?}", main_character.get_status());
        main_character.set_status(Death);
        println!("GameMainCharacter status after set status: {:?}", main_character.get_status());
        assert_eq!(main_character.get_health_point(), 150);

        let main_character = GameMainCharacter::new(200);
        println!("GameMainCharacter HealthPoint: {}", main_character.get_health_point());
        assert_eq!(main_character.get_health_point(), 200);
    }
}
