#[cfg(test)]
mod tests {
    use super::*;
    use tokio::test;
    use crate::redis::repository::redis_in_memory_repository::RedisInMemoryRepository;
    use crate::redis::repository::redis_in_memory_repository_impl::RedisInMemoryRepositoryImpl;

    #[tokio::test]
    async fn test_account_session_logout() {
        let mut redis_repository_mutex = RedisInMemoryRepositoryImpl::get_instance();
        let mut redis_repository_gaurd = redis_repository_mutex.lock().await;

        //redis_token 셋팅
        redis_repository_gaurd.set_permanent("test_logout_key", "test_logout_value").await;
        //redis_token key값을 인자로 받아서 삭제
        redis_repository_gaurd.del("test_logout_key").await;
    }
}