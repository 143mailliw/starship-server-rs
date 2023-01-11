#![allow(non_snake_case)]
use super::super::planet;
use super::super::planet_member::Model;
use super::super::planet_role;
use super::super::user;
use crate::errors;
use async_graphql::types::ID;
use async_graphql::{Context, Error, Object};
use chrono::NaiveDateTime;
use sea_orm::{DatabaseConnection, EntityTrait};

#[Object(name = "PlanetMember")]
impl Model {
    #[graphql(complexity = 0)]
    async fn id(&self) -> ID {
        ID(self.id.clone())
    }

    #[graphql(complexity = 5)]
    async fn planet(&self, ctx: &Context<'_>) -> Result<planet::Model, Error> {
        let db = ctx.data::<DatabaseConnection>().unwrap();

        match planet::Entity::find_by_id(self.planet.clone())
            .one(db)
            .await
        {
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

    #[graphql(complexity = 0)]
    async fn createdAt(&self) -> NaiveDateTime {
        self.created
    }
}
