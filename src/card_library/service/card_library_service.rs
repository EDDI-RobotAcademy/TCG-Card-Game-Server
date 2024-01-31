use async_trait::async_trait;

#[async_trait]
pub trait CardLibraryService {
    async fn open_library(&self);
}