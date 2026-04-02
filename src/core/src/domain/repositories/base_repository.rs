use super::super::errors::Error;
use async_trait::async_trait;

#[async_trait]
pub trait BaseRepository<T>: Sync + Send {
    async fn find_all(&self) -> Result<Vec<T>, Error>;
}
