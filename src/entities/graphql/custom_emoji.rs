use super::super::custom_emoji::Model;
use super::super::planet;
use super::super::user;
use crate::errors;
use async_graphql::types::ID;
use async_graphql::{Context, Error, Object};
use sea_orm::{DatabaseConnection, ModelTrait};

#[Object(
    name = "CustomEmoji",
    rename_fields = "camelCase",
    rename_args = "camelCase"
)]
impl Model {
    #[graphql(complexity = 0)]
    async fn id(&self) -> ID {
        ID(self.id.clone())
    }

    #[graphql(complexity = 5)]
    async fn owner(&self, ctx: &Context<'_>) -> Result<user::Model, Error> {
        let db = ctx.data::<DatabaseConnection>().unwrap();

        match self.find_related(user::Entity).one(db).await {
            Ok(value) => match value {
                Some(user) => Ok(user),
                None => Err(errors::create_internal_server_error(
                    None,
                    "OWNER_MISSING_ERROR",
                )),
            },
            Err(_error) => Err(errors::create_internal_server_error(
                None,
                "FIND_OWNER_ERROR",
            )),
        }
    }

    #[graphql(complexity = 5)]
    async fn planet(&self, ctx: &Context<'_>) -> Result<Option<planet::Model>, Error> {
        let db = ctx.data::<DatabaseConnection>().unwrap();

        match self.planet.clone() {
            Some(_id) => match self.find_related(planet::Entity).one(db).await {
                Ok(value) => match value {
                    Some(planet) => Ok(Some(planet)),
                    None => Err(errors::create_internal_server_error(
                        None,
                        "PLANET_MISSING_ERROR",
                    )),
                },
                Err(_error) => Err(errors::create_internal_server_error(
                    None,
                    "FIND_OWNER_ERROR",
                )),
            },
            None => Ok(None),
        }
    }

    #[graphql(complexity = 0)]
    async fn name(&self) -> String {
        self.name.clone()
    }

    #[graphql(complexity = 0)]
    async fn url(&self) -> String {
        self.url.clone()
    }
}
