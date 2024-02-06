use std::collections::HashMap;
use std::env;
use std::error::Error;
use std::fs::File;

pub fn build_dictionaries(csv_content: &Vec<Vec<String>>) -> (
    HashMap<String, String>,   // 종족
    HashMap<String, String>,   // 등급
    HashMap<String, String>,   // 종류(아이템, 서포트 등등)
    HashMap<String, String>,   // 필요한 에너지
    HashMap<String, String>,   // 공격력(ATK)
    HashMap<String, String>,   // 특성
    HashMap<String, String>,   // 스킬
    HashMap<String, String>,   // HP
) {
    let mut race_dictionary = HashMap::new();
    let mut card_grade_dictionary = HashMap::new();
    let mut card_kinds_dictionary = HashMap::new();
    let mut energy_needed_dictionary = HashMap::new();
    let mut attack_dictionary = HashMap::new();
    let mut passive_dictionary = HashMap::new();
    let mut skill_dictionary = HashMap::new();
    let mut hp_dictionary = HashMap::new();

    for record in csv_content.iter().skip(1) {
        let card_number = &record[6]; // 카드번호
        let race = &record[1]; // 종족
        let card_grade = &record[2]; // 등급
        let card_kind = &record[3]; // 종류
        let energy_needed = &record[9]; // 필요_에너지
        let attack = &record[10]; // 공격력
        let passive = &record[12]; // 패시브
        let skill = &record[13]; // 스킬
        let hp = &record[14]; // 체력

        race_dictionary.insert(card_number.clone(), race.clone());
        card_grade_dictionary.insert(card_number.clone(), card_grade.clone());
        card_kinds_dictionary.insert(card_number.clone(), card_kind.clone());
        energy_needed_dictionary.insert(card_number.clone(), energy_needed.clone());
        attack_dictionary.insert(card_number.clone(), attack.clone());
        passive_dictionary.insert(card_number.clone(), passive.clone());
        skill_dictionary.insert(card_number.clone(), skill.clone());
        hp_dictionary.insert(card_number.clone(), hp.clone());
    }

    (
        race_dictionary,
        card_grade_dictionary,
        card_kinds_dictionary,
        energy_needed_dictionary,
        attack_dictionary,
        passive_dictionary,
        skill_dictionary,
        hp_dictionary,
    )
}



// TODO: 아래의 카드 속성 dictionary 제작 기능은 card_data.csv 기반으로 변경
//             let card_number = &card_record[0]; // 카드번호
//             let name = &card_record[1]; // 이름
//             let race = &card_record[2]; // 종족
//             let grade = &card_record[3]; // 등급
//             let kind = &card_record[4]; // 종류
//             // 속성(병종)
//             let activation_energy = &card_record[6]; // 필요_에너지
//             let attack_point = &card_record[7]; // 공격력
//             let health_point = &card_record[8]; // 체력
//             // let passive = &record[12]; // 패시브
//             // let skill = &record[13]; // 스킬
//             // let hp = &record[14]; // 체력

pub fn build_card_kinds_dictionary(csv_content: &Vec<Vec<String>>) -> HashMap<i32, i32> {
    let mut card_kinds_dictionary: HashMap<i32, i32> = HashMap::new();

    for record in csv_content.iter() {
        let card_number = record[0].parse::<i32>();

        let card_kind = record[4].parse::<i32>();

        if card_number.is_ok() && card_kind.is_ok()  {
            card_kinds_dictionary.insert(card_number.unwrap(), card_kind.unwrap());
        } else {
            eprintln!("Failed to parse card number : {:?}", record[0]);
            eprintln!("Failed to parse card kinds : {:?}", record[4]);
        }
    }

    card_kinds_dictionary
}

pub fn build_card_grade_dictionary(csv_content: &Vec<Vec<String>>) -> HashMap<i32, i32> {
    let mut card_grade_dictionary: HashMap<i32, i32> = HashMap::new();

    for record in csv_content.iter() {
        let card_number = record[0].parse::<i32>();

        let card_grade = record[3].parse::<i32>();

        if card_number.is_ok() && card_grade.is_ok() {
            card_grade_dictionary.insert(card_number.unwrap(), card_grade.unwrap());
        } else {
            eprintln!("Failed to parse card number: {:?}", record[0]);
            eprintln!("Failed to parse card grade: {:?}", record[3]);
        }
    }

    card_grade_dictionary
}

pub fn build_card_race_dictionary(csv_content: &Vec<Vec<String>>) -> HashMap<i32, i32> {
    let mut card_race_dictionary = HashMap::new();

    for record in csv_content.iter() {
        let card_number = record[0].parse::<i32>();

        let card_race = record[2].parse::<i32>();

        if card_number.is_ok() && card_race.is_ok() {
            card_race_dictionary.insert(card_number.unwrap(), card_race.unwrap());
        } else {
            eprintln!("Failed to parse card number: {:?}", record[0]);
            eprintln!("Failed to parse card race: {:?}", record[2]);
        }
    }

    card_race_dictionary
}


// 카드 종류(서포트, 아이템 등등)
pub fn get_card_kinds<'a>(
    card_number: &'a str,
    card_kinds_dictionary: &'a HashMap<String, String>,
) -> Option<&'a String> {
    card_kinds_dictionary.get(card_number)
}

// 카드 종족(휴먼, 언데드, 트랜트 등등)
pub fn get_race<'a>(
    card_number: &'a str,
    race_dictionary: &'a HashMap<String, String>,
) -> Option<&'a String> {
    race_dictionary.get(card_number)
}

// 카드 등급
pub fn get_card_grade<'a>(
    card_number: &'a str,
    card_grade_dictionary: &'a HashMap<String, String>,
) -> Option<&'a String> {
    card_grade_dictionary.get(card_number)
}

// 카드의 필요 에너지
pub fn get_energy_needed<'a>(
    card_number: &'a str,
    energy_needed_dictionary: &'a HashMap<String, String>,
) -> Option<&'a String> {
    energy_needed_dictionary.get(card_number)
}

// 카드 공격력
pub fn get_attack<'a>(
    card_number: &'a str,
    attack_dictionary: &'a HashMap<String, String>,
) -> Option<&'a String> {
    attack_dictionary.get(card_number)
}

// TODO: 추후 특성1, 2, 3에 따라 다 쪼개야합니다
// 카드 특성
pub fn get_passive<'a>(
    card_number: &'a str,
    passive_dictionary: &'a HashMap<String, String>,
) -> Option<&'a String> {
    passive_dictionary.get(card_number)
}

// TODO: 추후 스킬1, 2, 3에 따라 다 쪼개야합니다
// 카드 스킬
pub fn get_skill<'a>(
    card_number: &'a str,
    skill_dictionary: &'a HashMap<String, String>,
) -> Option<&'a String> {
    skill_dictionary.get(card_number)
}

// 카드 HP
pub fn get_hp<'a>(card_number: &'a str, hp_dictionary: &'a HashMap<String, String>) -> Option<&'a String> {
    hp_dictionary.get(card_number)
}

pub fn extract_csv_header(file_path: &str) -> Result<Vec<String>, Box<dyn Error>> {
    let current_exe_path = env::current_exe()?;
    let current_dir = current_exe_path.parent().ok_or("Unable to get parent directory")?;

    let absolute_path = current_dir.join(file_path);
    println!("absolute_path: {:?}", absolute_path);

    let file = File::open(absolute_path)?;

    let mut rdr = csv::Reader::from_reader(file);
    let header = rdr.headers()?.iter().map(|field| field.to_string()).collect();

    Ok(header)
}

pub fn csv_read(file_path: &str) -> Result<Vec<Vec<String>>, Box<dyn Error>> {
    let current_exe_path = env::current_exe()?;
    let current_dir = current_exe_path.parent().ok_or("Unable to get parent directory")?;

    let absolute_path = current_dir.join(file_path);
    println!("absolute_path: {:?}", absolute_path);

    let file = File::open(absolute_path)?;

    let mut rdr = csv::Reader::from_reader(file);
    let mut csv_content: Vec<Vec<String>> = Vec::new();

    for result in rdr.records() {
        let record = result?;
        let record_values: Vec<String> = record.iter().map(|field| field.to_string()).collect();

        csv_content.push(record_values);
    }

    Ok(csv_content)
}

#[cfg(test)]
mod tests {
    use std::env;
    use super::*;

    #[test]
    fn test_extract_csv_header() {
        let filename = "../../../resources/csv/every_card.csv";
        match extract_csv_header(filename) {
            Ok(header) => {
                println!("Header successfully extracted.");
                println!("Header: {:?}", header);
            }
            Err(err) => eprintln!("Error: {}", err),
        }
    }

    #[test]
    fn test_build_card_kinds_dictionary() {
        let filename = "../../../resources/csv/every_card.csv";
        match csv_read(filename) {
            Ok(csv_content) => {
                let card_kinds_dictionary = build_card_kinds_dictionary(&csv_content);
                println!("Card Kinds Dictionary: {:?}", card_kinds_dictionary);
            }
            Err(err) => eprintln!("Error: {}", err),
        }
    }

    #[test]
    fn test_build_card_kinds_dictionary_with_number() {
        let filename = "../../../resources/csv/every_card.csv";
        match csv_read(filename) {
            Ok(csv_content) => {
                let card_kinds_dictionary = build_card_kinds_dictionary(&csv_content);
                let specific_card_number = 2;

                assert!(card_kinds_dictionary.contains_key(&specific_card_number));
                println!("Card Kinds Dictionary: {:?}", card_kinds_dictionary);
            }
            Err(err) => eprintln!("Error: {}", err),
        }
    }

    #[test]
    fn test_read_csv() {
        let current_exe_path = env::current_exe();

        match current_exe_path {
            Ok(path) => println!("현재 실행 중인 프로그램의 경로: {:?}", path),
            Err(e) => eprintln!("오류 발생: {:?}", e),
        }

        let filename = "../../../resources/csv/every_card.csv";
        match csv_read(filename) {
            Ok(csv_content) => {
                println!("CSV file successfully processed.");
                for record in csv_content {
                    println!("{:?}", record);
                }
            }
            Err(err) => eprintln!("Error: {}", err),
        }
    }

    #[test]
    fn test_build_dictionaries() {
        let filename = "../../../resources/csv/every_card.csv";
        match csv_read(filename) {
            Ok((csv_content)) => {
                println!("CSV file successfully processed.");

                let (
                    race_dictionary,
                    card_grade_dictionary,
                    card_kinds_dictionary,
                    energy_needed_dictionary,
                    attack_dictionary,
                    passive_dictionary,
                    skill_dictionary,
                    hp_dictionary,
                ) = build_dictionaries(&csv_content);

                println!("race_dictionary: {:?}", race_dictionary);
                println!("card_grade_dictionary: {:?}", card_grade_dictionary);
                println!("card_kinds_dictionary: {:?}", card_kinds_dictionary);
                println!("energy_needed_dictionary: {:?}", energy_needed_dictionary);
                println!("attack_dictionary: {:?}", attack_dictionary);
                println!("passive_dictionary: {:?}", passive_dictionary);
                println!("skill_dictionary: {:?}", skill_dictionary);
                println!("hp_dictionary: {:?}", hp_dictionary);

                let card_number_to_test = "5"; // Modify this to the desired card number

                match get_card_kinds(card_number_to_test, &card_kinds_dictionary) {
                    Some(card_kinds) => println!("Card Kinds for {} : {}", card_number_to_test, card_kinds),
                    None => println!("Card not found: {}", card_number_to_test),
                }
            }
            Err(err) => eprintln!("Error: {}", err),
        }
    }
}