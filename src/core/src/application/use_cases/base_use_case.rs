use super::super::exceptions::Exception;
use async_trait::async_trait;

#[async_trait]
pub trait UseCase<Input, Output> {
    async fn handle(&self, input: Input) -> Result<Output, Exception>;
}
