use tokio::task::LocalSet;
use std::borrow::BorrowMut;
use tokio::io::{AsyncWriteExt, stdout};
use tokio::time::{sleep, Duration};

pub trait ConceptTestRepository {
    fn get_data(&self) -> String;
    fn increment_count(&mut self);
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

impl ConceptTestRepository for ConceptTestRepositoryImpl {
    fn get_data(&self) -> String {
        format!("{} - Count: {}", self.data, 0)
    }

    fn increment_count(&mut self) {
    }
}

pub trait ConceptTestService {
    fn perform_action(&self, thread_id: usize);
}

pub struct ConceptTestServiceImpl;

impl ConceptTestServiceImpl {
    pub fn new() -> Self {
        ConceptTestServiceImpl
    }
}

impl ConceptTestService for ConceptTestServiceImpl {
    fn perform_action(&self, thread_id: usize) {
        LOCAL_REPOSITORY.with(|mut repository| {
            let mut repository = repository.borrow_mut();
            let mut count = 0;
            for _ in 0..1000 {
                count += 1;
                let output = format!("Thread {}: Result: {} - Count: {}", thread_id, repository.get_data(), count);
                println!("{}", output);

                tokio::spawn(async {
                    sleep(Duration::from_millis(10)).await;
                    let _ = stdout().flush().await;
                });
            }
        });
    }
}

thread_local! {
    static LOCAL_REPOSITORY: ConceptTestRepositoryImpl =
        ConceptTestRepositoryImpl::new("Hello, Repository!");
}

#[tokio::test]
async fn thread_test() {
    env_logger::init();
    let local_set = LocalSet::new();
    let mut handles = Vec::new();

    local_set
        .run_until(async {
            for i in 0..10 {
                let handle = tokio::task::spawn_blocking(move || {
                    let service = ConceptTestServiceImpl::new();
                    service.perform_action(i);
                });
                handles.push(handle);
            }
        })
        .await;

    for handle in handles {
        handle.await.expect("Thread join failed");
    }
}