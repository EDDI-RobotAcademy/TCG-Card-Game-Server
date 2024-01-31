use rand::prelude::SliceRandom;
use std::fmt;
use std::collections::HashMap;
use std::env;
use std::error::Error;
use std::fs::File;
use std::path::Path;
use crate::common::csv::csv_reader::{build_dictionaries, csv_read};


pub fn get_card_name<'a>(
    card_number: &'a str,
    card_name_dictionary: &'a HashMap<String, String>,
) -> Option<&'a String> {
    card_name_dictionary.get(card_number)
}
pub fn build_dictionaries2(csv_content: &Vec<Vec<String>>) -> (
    HashMap<String, String>,   // 카드명
    HashMap<String, String>,   // 종족
    HashMap<String, String>,   // 등급
    HashMap<String, String>,   // 종류(아이템, 서포트 등등)
    HashMap<String, String>,   // 필요한 에너지
    HashMap<String, String>,   // 공격력(ATK)
    HashMap<String, String>,   // 특성
    HashMap<String, String>,   // 스킬
    HashMap<String, String>,   // HP
) {
    let mut card_name_dictionary =HashMap::new();
    let mut race_dictionary = HashMap::new();
    let mut card_grade_dictionary = HashMap::new();
    let mut card_kinds_dictionary = HashMap::new();
    let mut energy_needed_dictionary = HashMap::new();
    let mut attack_dictionary = HashMap::new();
    let mut passive_dictionary = HashMap::new();
    let mut skill_dictionary = HashMap::new();
    let mut hp_dictionary = HashMap::new();

    for record in csv_content.iter().skip(1) {
        let card_name=&record[0];   //카드명
        let card_number = &record[6]; // 카드번호
        let race = &record[1]; // 종족
        let card_grade = &record[2]; // 등급
        let card_kind = &record[3]; // 종류
        let energy_needed = &record[9]; // 필요_에너지
        let attack = &record[10]; // 공격력
        let passive = &record[12]; // 패시브
        let skill = &record[13]; // 스킬
        let hp = &record[14]; // 체력

        card_name_dictionary.insert(card_number.clone(), card_name.clone());
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
        card_name_dictionary,
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






pub fn csv_read2(file_path: &str) -> Result<Vec<Vec<String>>, Box<dyn Error>> {
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
    use std::collections::HashMap;
    use std::env;
    use std::error::Error;
    use std::fs::File;
    use rand::prelude::SliceRandom;
    use crate::common::csv::csv_reader::{build_dictionaries, csv_read, get_card_kinds};
    use crate::ugly_tests::ingame::test_express_card_use_csv::{build_dictionaries2, get_card_name};


    #[test]
    fn express_card_use_csv() {
        let card_name_dictionary;
        let race_dictionary;
        let card_grade_dictionary;
        let card_kinds_dictionary;
        let energy_needed_dictionary;
        let attack_dictionary;
        let passive_dictionary;
        let skill_dictionary;
        let hp_dictionary;

        let filename = "../../../resources/csv/every_card.csv";
        let mut csv_content2: Vec<Vec<String>> = Vec::new();
        match csv_read(filename) {
            Ok(csv_content) => {
                csv_content2=csv_content.clone();
                println!("CSV file successfully processed.");
                for record in csv_content {
                    //println!("{:?}", record);
                }
            }
            Err(err) => eprintln!("Error: {}", err),
        }
        (
            card_name_dictionary,
            race_dictionary,
            card_grade_dictionary,
            card_kinds_dictionary,
            energy_needed_dictionary,
            attack_dictionary,
            passive_dictionary,
            skill_dictionary,
            hp_dictionary,
        )=build_dictionaries2(&csv_content2);
        let mut card_number="171";
        match get_card_name(card_number, &card_name_dictionary) {
            Some(card_name) => println!("Card Kinds for {} : {}", card_number, card_name),
            None => println!("Card not found: {}", card_number),
        }

    }
}

// let deck_list_tuples = vec![
//     (58, 2),   (164, 3),   (171, 2), (160, 2),   //58 신화 164 일반 171 신화 160 전설  121 영웅 103 언커먼
//     (121,3),    (103,3)
// ];
