use tokio::sync::Mutex;
use async_trait::async_trait;
use std::sync::Arc;

#[async_trait]
pub trait ConceptTestRepository {
    async fn get_data(&self) -> String;
}

pub struct ConceptTestRepositoryImpl {
    data: String,
}

impl ConceptTestRepositoryImpl {
    pub fn new(data: &str) -> Self {
        ConceptTestRepositoryImpl {
            data: data.to_string(),
        }
    }
}

#[async_trait]
impl ConceptTestRepository for ConceptTestRepositoryImpl {
    async fn get_data(&self) -> String {
        self.data.clone()
    }
}

#[tokio::test]
async fn thread_test() {
    let shared_repository = Arc::new(Mutex::new(ConceptTestRepositoryImpl::new("Hello, Repository!")));

    let mut handles = vec![];

    for _ in 0..10 {
        let repository_clone = Arc::clone(&shared_repository);
        let handle = tokio::spawn(async move {
            let repository = repository_clone.lock().await;
            let data = repository.get_data().await;

            println!("Data from repository: {}", data);
        });

        handles.push(handle);
    }

    for handle in handles {
        handle.await.unwrap();
    }
}
