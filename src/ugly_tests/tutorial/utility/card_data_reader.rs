use std::collections::HashMap;
use std::env;
use std::error::Error;
use std::fs::File;

pub fn card_data_csv_read(file_path: &str) -> Result<Vec<Vec<String>>, Box<dyn Error>> {
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

pub fn build_card_data_dictionaries(csv_content: &Vec<Vec<String>>) -> (
    HashMap<String, String>,   // name
    HashMap<String, String>,   // race
    HashMap<String, String>,   // grade
    HashMap<String, String>,   // kinds
    HashMap<String, String>,   // activation_energy
    HashMap<String, String>,   // attack_point
    HashMap<String, String>,   // health_point
    // HashMap<String, String>,   // passive1
    // HashMap<String, String>,   // passive2
    // HashMap<String, String>,   // passive3
    // HashMap<String, String>,   // skill1
    // HashMap<String, String>,   // skill2
    // HashMap<String, String>,   // skill2
    // HashMap<String, String>,   // effect1
    // HashMap<String, String>,   // effect2
    // HashMap<String, String>,   // effect3
) {
    let mut name_dictionary = HashMap::new();
    let mut race_dictionary = HashMap::new();
    let mut grade_dictionary = HashMap::new();
    let mut kinds_dictionary = HashMap::new();
    let mut activation_energy_dictionary = HashMap::new();
    let mut attack_point_dictionary = HashMap::new();
    let mut health_point_dictionary = HashMap::new();
    // let mut passive1_dictionary = HashMap::new();
    // let mut passive2_dictionary = HashMap::new();
    // let mut passive3_dictionary = HashMap::new();
    // let mut skill1_dictionary = HashMap::new();
    // let mut skill2_dictionary = HashMap::new();
    // let mut skill3_dictionary = HashMap::new();
    // let mut effect1_dictionary = HashMap::new();
    // let mut effect2_dictionary = HashMap::new();
    // let mut effect3_dictionary = HashMap::new();

    for card_record in csv_content.iter() {
        let card_number = &card_record[0]; // 카드번호
        let name = &card_record[1]; // 이름
        let race = &card_record[2]; // 종족
        let grade = &card_record[3]; // 등급
        let kind = &card_record[4]; // 종류
        // 속성(병종)
        let activation_energy = &card_record[6]; // 필요_에너지
        let attack_point = &card_record[7]; // 공격력
        let health_point = &card_record[8]; // 체력
        // let passive = &record[12]; // 패시브
        // let skill = &record[13]; // 스킬
        // let hp = &record[14]; // 체력

        name_dictionary.insert(card_number.clone(), name.clone());
        race_dictionary.insert(card_number.clone(), race.clone());
        grade_dictionary.insert(card_number.clone(), grade.clone());
        kinds_dictionary.insert(card_number.clone(), kind.clone());
        activation_energy_dictionary.insert(card_number.clone(), activation_energy.clone());
        attack_point_dictionary.insert(card_number.clone(), attack_point.clone());
        health_point_dictionary.insert(card_number.clone(), health_point.clone());
        // passive_dictionary.insert(card_number.clone(), passive.clone());
        // skill_dictionary.insert(card_number.clone(), skill.clone());
        // hp_dictionary.insert(card_number.clone(), hp.clone());
    }

    (
        name_dictionary,
        race_dictionary,
        grade_dictionary,
        kinds_dictionary,
        activation_energy_dictionary,
        attack_point_dictionary,
        health_point_dictionary
    )
}
// 카드 이름("넘쳐흐르는 사기" 등)
pub fn get_name<'a>(
    card_number: &'a str,
    name_dictionary: &'a HashMap<String, String>,
) -> Option<&'a String> { name_dictionary.get(card_number) }

// 카드 종족(휴먼, 언데드, 트랜트 등등)
pub fn get_race<'a>(
    card_number: &'a str,
    race_dictionary: &'a HashMap<String, String>,
) -> Option<&'a String> {
    race_dictionary.get(card_number)
}

// 카드 종류(서포트, 아이템 등등)
pub fn get_kind<'a>(
    card_number: &'a str,
    kinds_dictionary: &'a HashMap<String, String>,
) -> Option<&'a String> {
    kinds_dictionary.get(card_number)
}

// 카드 등급
pub fn get_grade<'a>(
    card_number: &'a str,
    grade_dictionary: &'a HashMap<String, String>,
) -> Option<&'a String> {
    grade_dictionary.get(card_number)
}

// 카드의 필요(활성화) 에너지
pub fn get_activation_energy<'a>(
    card_number: &'a str,
    activation_energy_dictionary: &'a HashMap<String, String>,
) -> Option<&'a String> {
    activation_energy_dictionary.get(card_number)
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
// pub fn get_passive<'a>(
//     card_number: &'a str,
//     passive_dictionary: &'a HashMap<String, String>,
// ) -> Option<&'a String> {
//     passive_dictionary.get(card_number)
// }
//
// // TODO: 추후 스킬1, 2, 3에 따라 다 쪼개야합니다
// // 카드 스킬
// pub fn get_skill<'a>(
//     card_number: &'a str,
//     skill_dictionary: &'a HashMap<String, String>,
// ) -> Option<&'a String> {
//     skill_dictionary.get(card_number)
// }
//
// // 카드 HP
// pub fn get_hp<'a>(card_number: &'a str, hp_dictionary: &'a HashMap<String, String>) -> Option<&'a String> {
//     hp_dictionary.get(card_number)
// }