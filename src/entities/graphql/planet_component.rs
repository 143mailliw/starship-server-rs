use super::super::planet;
use super::super::planet_component::Model;
use crate::errors;
use async_graphql::types::ID;
use async_graphql::{Context, Error, Object};
use chrono::NaiveDateTime;
use sea_orm::{DatabaseConnection, ModelTrait};

#[Object(
    name = "PlanetComponent",
    rename_fields = "camelCase",
    rename_args = "camelCase"
)]
impl Model {
    #[graphql(complexity = 0)]
    async fn id(&self) -> ID {
        ID(self.id.clone())
    }

    #[graphql(complexity = 0)]
    async fn name(&self) -> &String {
        &self.name
    }

    #[graphql(complexity = 0)]
    async fn created_at(&self) -> NaiveDateTime {
        self.created
    }

    #[graphql(complexity = 0)]
    async fn component_id(&self) -> &String {
        &self.component_id
    }

    #[graphql(complexity = 0)]
    async fn r#type(&self) -> &String {
        &self.r#type
    }

    #[graphql(complexity = 5)]
    async fn planet(&self, ctx: &Context<'_>) -> Result<planet::Model, Error> {
        let db = ctx.data::<DatabaseConnection>().unwrap();

        self.find_related(planet::Entity)
            .one(db)
            .await
            .map_err(|_| errors::create_internal_server_error(None, "FIND_PLANET_ERROR"))?
            .ok_or(errors::create_internal_server_error(
                None,
                "PLANET_MISSING_ERROR",
            ))
    }
}
