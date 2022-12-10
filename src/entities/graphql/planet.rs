#![allow(non_snake_case)]
use super::super::custom_emoji;
use super::super::planet::Model;
use super::super::user;
use crate::errors;
use async_graphql::types::ID;
use async_graphql::{Context, Error, Object};
use chrono::NaiveDateTime;
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};

#[Object(name = "Planet")]
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

    #[graphql(complexity = 0)]
    async fn private(&self) -> bool {
        self.private
    }

    #[graphql(complexity = 0)]
    async fn memberCount(&self) -> i32 {
        self.member_count
    }

    // TODO: components
    // TODO: homeComponent

    #[graphql(complexity = 0)]
    async fn featured(&self) -> bool {
        self.featured
    }

    #[graphql(complexity = 0)]
    async fn verified(&self) -> bool {
        self.verified
    }

    #[graphql(complexity = 0)]
    async fn partnered(&self) -> bool {
        self.partnered
    }

    // TODO: members

    #[graphql(complexity = 0)]
    async fn featuredDescription(&self) -> String {
        self.featured_description.clone()
    }

    // TODO: invites

    #[graphql(complexity = 5)]
    async fn banned(&self, ctx: &Context<'_>) -> Result<Vec<user::Model>, Error> {
        // TODO: potentially lock this behind a permission
        let db = ctx.data::<DatabaseConnection>().unwrap();

        match user::Entity::find()
            .filter(user::Column::Id.is_in(self.banned.clone()))
            .all(db)
            .await
        {
            Ok(value) => Ok(value),
            Err(_error) => Err(errors::create_internal_server_error(
                None,
                "FIND_BANNED_ERROR",
            )),
        }
    }

    #[graphql(complexity = 0)]
    async fn css(&self) -> String {
        self.css.clone()
    }

    #[graphql(complexity = 0)]
    async fn description(&self) -> Option<String> {
        self.description.clone()
    }

    #[graphql(complexity = 5)]
    async fn customEmojis(&self, ctx: &Context<'_>) -> Result<Vec<custom_emoji::Model>, Error> {
        let db = ctx.data::<DatabaseConnection>().unwrap();

        match custom_emoji::Entity::find()
            .filter(custom_emoji::Column::Planet.eq(self.id.clone()))
            .all(db)
            .await
        {
            Ok(value) => Ok(value),
            Err(_error) => Err(errors::create_internal_server_error(
                None,
                "FIND_EMOJIS_ERROR",
            )),
        }
    }

    // TODO: unread
    // TODO: mentioned
}
