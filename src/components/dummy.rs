use super::component;
use async_graphql::Error;
use async_trait::async_trait;

#[derive(Default)]
pub struct Component;

#[async_trait]
impl component::Trait for Component {
    async fn create() -> Result<String, Error> {
        Ok("dummy".to_string())
    }
    async fn delete(_id: String) -> Result<bool, Error> {
        Ok(true)
    }
}
