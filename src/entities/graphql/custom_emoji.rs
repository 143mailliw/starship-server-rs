#![allow(non_snake_case)]
use super::super::custom_emoji::Model;
use super::super::planet;
use super::super::user;
use crate::errors;
use async_graphql::types::ID;
use async_graphql::{Context, Error, Object};
use sea_orm::{DatabaseConnection, EntityTrait};

#[Object(name = "CustomEmoji")]
impl Model {
    #[graphql(complexity = 0)]
    async fn id(&self) -> ID {
        ID(self.id.clone())
    }

    #[graphql(complexity = 5)]
    async fn owner(&self, ctx: &Context<'_>) -> Result<user::Model, Error> {
        let db = ctx.data::<DatabaseConnection>().unwrap();

        match user::Entity::find_by_id(self.owner.clone()).one(db).await {
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
            Some(id) => match planet::Entity::find_by_id(id).one(db).await {
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

    #[graphql(
        deprecation = "Field is redundant, consider using owner instead",
        complexity = 5
    )]
    async fn user(&self, ctx: &Context<'_>) -> Result<user::Model, Error> {
        self.owner(ctx).await
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
