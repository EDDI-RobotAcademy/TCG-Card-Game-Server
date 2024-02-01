#[derive(Debug, Clone, Copy)]
enum NumberEnum {
    Zero = 0,
    One = 1,
    Two = 2,
    Three = 3,
}

#[cfg(test)]
mod tests {
    use super::NumberEnum;

    #[test]
    fn test_number_enum() {
        let zero_value = NumberEnum::Zero;
        println!("Zero value: {:?}", zero_value);
        assert_eq!(zero_value as i32, 0);

        let one_value = NumberEnum::One;
        println!("One value: {:?}", one_value);
        assert_eq!(one_value as i32, 1);

        let two_value = NumberEnum::Two;
        println!("Two value: {:?}", two_value);
        assert_eq!(two_value as i32, 2);

        let three_value = NumberEnum::Three;
        println!("Three value: {:?}", three_value);
        assert_eq!(three_value as i32, 3);
    }

    #[test]
    fn test_number_enum_value() {
        // 각 variant에 대한 테스트 로직
        let zero_value = NumberEnum::Zero;
        println!("Zero value: {}", zero_value as i32);
        assert_eq!(zero_value as i32, 0);

        let one_value = NumberEnum::One;
        println!("One value: {}", one_value as i32);
        assert_eq!(one_value as i32, 1);

        let two_value = NumberEnum::Two;
        println!("Two value: {}", two_value as i32);
        assert_eq!(two_value as i32, 2);

        let three_value = NumberEnum::Three;
        println!("Three value: {}", three_value as i32);
        assert_eq!(three_value as i32, 3);
    }
}
