use std::env;
use std::error::Error;
use std::fs::File;
use std::path::Path;

fn extract_csv_header(file_path: &str) -> Result<Vec<String>, Box<dyn Error>> {
    let current_exe_path = env::current_exe()?;
    let current_dir = current_exe_path.parent().ok_or("Unable to get parent directory")?;
    
    let absolute_path = current_dir.join(file_path);
    println!("absolute_path: {:?}", absolute_path);

    let file = File::open(absolute_path)?;

    let mut rdr = csv::Reader::from_reader(file);
    let header = rdr.headers()?.iter().map(|field| field.to_string()).collect();

    Ok(header)
}

fn csv_read(file_path: &str) -> Result<Vec<Vec<String>>, Box<dyn Error>> {
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
}