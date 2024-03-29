use super::super::planet;
use super::super::planet_member::Model;
use super::super::planet_role;
use super::super::user;
use crate::errors;
use async_graphql::types::ID;
use async_graphql::{Context, Error, Object};
use chrono::NaiveDateTime;
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, ModelTrait, QueryFilter};

#[Object(
    name = "PlanetMember",
    rename_fields = "camelCase",
    rename_args = "camelCase"
)]
impl Model {
    #[graphql(complexity = 0)]
    async fn id(&self) -> ID {
        ID(self.id.clone())
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

    #[graphql(complexity = 5)]
    async fn user(&self, ctx: &Context<'_>) -> Result<user::Model, Error> {
        let db = ctx.data::<DatabaseConnection>().unwrap();

        self.find_related(user::Entity)
            .one(db)
            .await
            .map_err(|_| errors::create_internal_server_error(None, "FIND_USER_ERROR"))?
            .ok_or(errors::create_internal_server_error(
                None,
                "USER_MISSING_ERROR",
            ))
    }

    #[graphql(complexity = 5)]
    async fn roles(&self, ctx: &Context<'_>) -> Result<Vec<planet_role::Model>, Error> {
        let db = ctx.data::<DatabaseConnection>().unwrap();

        planet_role::Entity::find()
            .filter(planet_role::Column::Id.is_in(self.roles.clone()))
            .all(db)
            .await
            .map_err(|_| errors::create_internal_server_error(None, "FIND_ROLES_ERROR"))
    }

    #[graphql(complexity = 0)]
    async fn permissions(&self) -> &Vec<String> {
        &self.permissions
    }

    #[graphql(complexity = 0)]
    async fn created_at(&self) -> NaiveDateTime {
        self.created
    }

    #[graphql(complexity = 0)]
    async fn banned(&self) -> bool {
        self.banned
    }
}
