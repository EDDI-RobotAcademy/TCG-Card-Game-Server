#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum RaceEnumValue {
    Undead = 1,
    Human = 2,
    Trent = 3,
    Angel = 4,
    Machine = 5,
    Chaos = 6,
}

#[test]
fn test_race_enum_value() {
    let undead = RaceEnumValue::Undead as i32;
    println!("undead: {}", undead);
    assert_eq!(undead, 1);

    let human = RaceEnumValue::Human as i32;
    println!("human: {}", human);
    assert_eq!(human, 2);

    let trent = RaceEnumValue::Trent as i32;
    println!("trent: {}", trent);
    assert_eq!(trent, 3);

    let angel = RaceEnumValue::Angel as i32;
    println!("trent: {}", angel);
    assert_eq!(angel, 4);

    let machine = RaceEnumValue::Machine as i32;
    println!("machine: {}", machine);
    assert_eq!(machine, 5);

    let chaos = RaceEnumValue::Chaos as i32;
    println!("chaos: {}", chaos);
    assert_eq!(chaos, 6);
}
