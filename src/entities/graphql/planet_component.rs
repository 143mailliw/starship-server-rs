#![allow(non_snake_case)]
use super::super::planet;
use super::super::planet_component::Model;
use crate::errors;
use async_graphql::types::ID;
use async_graphql::{Context, Error, Object};
use chrono::NaiveDateTime;
use sea_orm::{DatabaseConnection, ModelTrait};

#[Object(name = "PlanetComponent")]
impl Model {
    #[graphql(complexity = 0)]
    async fn id(&self) -> ID {
        ID(self.id.clone())
    }

    #[graphql(complexity = 0)]
    async fn name(&self) -> String {
        self.name.clone()
    }

    #[graphql(complexity = 0)]
    async fn createdAt(&self) -> NaiveDateTime {
        self.created
    }

    #[graphql(complexity = 0)]
    async fn componentId(&self) -> String {
        self.component_id.clone()
    }

    #[graphql(complexity = 0)]
    async fn r#type(&self) -> String {
        self.r#type.clone()
    }

    #[graphql(complexity = 5)]
    async fn planet(&self, ctx: &Context<'_>) -> Result<planet::Model, Error> {
        let db = ctx.data::<DatabaseConnection>().unwrap();

        match self.find_related(planet::Entity).one(db).await {
            Ok(value) => match value {
                Some(planet) => Ok(planet),
                None => Err(errors::create_internal_server_error(
                    None,
                    "PLANET_MISSING_ERROR",
                )),
            },
            Err(_error) => Err(errors::create_internal_server_error(
                None,
                "FIND_PLANET_ERROR",
            )),
        }
    }
}
