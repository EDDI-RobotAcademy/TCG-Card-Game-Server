use std::cell::RefCell;
thread_local! {
    static LOCAL_REPOSITORY: RefCell<ConceptTestRepositoryImpl> =
        RefCell::new(ConceptTestRepositoryImpl::new("Hello, Repository!"));
}

pub trait ConceptTestRepository {
    fn get_data(&self) -> String;
}

pub struct ConceptTestRepositoryImpl {
    data: String,
    count: u32,
}

impl ConceptTestRepositoryImpl {
    pub fn new(data: &str) -> Self {
        ConceptTestRepositoryImpl {
            data: data.to_string(),
            count: 0,
        }
    }

    pub fn increment_count(&mut self) {
        self.count += 1;
    }
}

impl ConceptTestRepository for ConceptTestRepositoryImpl {
    fn get_data(&self) -> String {
        format!("{} - Count: {}", self.data, self.count)
    }
}

pub trait ConceptTestService {
    fn perform_action(&self) -> String;
}

pub struct ConceptTestServiceImpl;

impl ConceptTestServiceImpl {
    pub fn new() -> Self {
        ConceptTestServiceImpl
    }
}

impl ConceptTestService for ConceptTestServiceImpl {
    fn perform_action(&self) -> String {
        LOCAL_REPOSITORY.with(|r| {
            let mut repository = r.borrow_mut();
            repository.increment_count();
            let data = repository.get_data();
            println!("Result: {}", data);
            "Task completed".to_string()
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::ugly_tests::thread::test_no_singleton_thread_service::{ConceptTestService, ConceptTestServiceImpl};

    #[test]
    fn test_threads() {
        let handles: Vec<_> = (0..10)
            .map(|_| {
                std::thread::spawn(|| {
                    let service = ConceptTestServiceImpl::new();
                    let result = service.perform_action();
                    println!("Service Result: {}", result);
                })
            })
            .collect();

        for handle in handles {
            handle.join().unwrap();
        }
    }
}

