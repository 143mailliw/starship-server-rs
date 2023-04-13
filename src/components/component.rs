use async_graphql::Error;
use async_trait::async_trait;

#[async_trait]
pub trait Trait {
    async fn create() -> Result<String, Error>;
    async fn delete(id: String) -> Result<bool, Error>;
}
