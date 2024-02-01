use std::path::{Path, PathBuf};

pub struct RootPath;

impl RootPath {
    fn find_project_root() -> Option<PathBuf> {
        let current_dir = std::env::current_dir().ok()?;

        let mut current_path = current_dir.clone(); // 복제하여 새로운 소유권을 만듭니다.
        while !current_path.join("Cargo.toml").exists() {
            if let Some(parent) = current_path.parent() {
                current_path = parent.to_path_buf(); // 혹은 `parent.into()`로도 사용 가능합니다.
            } else {
                return None;
            }
        }

        Some(current_path)
    }

    pub fn make_full_path(filename: &str) -> Option<PathBuf> {
        if let Some(project_root) = Self::find_project_root() {
            let full_path = project_root.join(filename);
            Some(full_path)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_find_project_root() {
        if let Some(project_root) = RootPath::find_project_root() {
            println!("프로젝트 최상위 디렉토리: {:?}", project_root);
        } else {
            println!("프로젝트 최상위 디렉토리를 찾을 수 없습니다.");
        }
    }

    #[test]
    fn test_find_real_file() {
        if let Some(project_root) = RootPath::find_project_root() {
            println!("프로젝트 최상위 디렉토리: {:?}", project_root);

            let csv_path = project_root.join("resources/csv/every_card.csv");

            if fs::metadata(&csv_path).is_ok() {
                println!("파일이 존재합니다: {:?}", csv_path);
            } else {
                println!("파일이 존재하지 않습니다: {:?}", csv_path);
            }
        } else {
            println!("프로젝트 최상위 디렉토리를 찾을 수 없습니다.");
        }
    }

    #[test]
    fn test_make_full_path() {
        if let Some(full_path) = RootPath::make_full_path("resources/csv/every_card.csv") {
            println!("절대 경로: {:?}", full_path);
        } else {
            println!("프로젝트 최상위 디렉토리를 찾을 수 없습니다.");
        }
    }
}