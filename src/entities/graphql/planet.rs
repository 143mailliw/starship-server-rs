#![allow(non_snake_case)]
use super::super::planet::Model;
use super::super::user;
use crate::errors;
use crate::sessions::Session;
use async_graphql::types::ID;
use async_graphql::{Context, Error, Object};
use chrono::NaiveDateTime;
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};

#[Object(name = "Planet")]
impl Model {
    async fn id(&self) -> ID {
        ID(self.id.clone())
    }

    async fn name(&self) -> String {
        self.name.clone()
    }

    async fn createdAt(&self) -> NaiveDateTime {
        self.created
    }

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
            Err(error) => Err(errors::create_internal_server_error(
                None,
                "FIND_OWNER_ERROR",
            )),
        }
    }

    async fn private(&self) -> bool {
        self.private
    }

    async fn followerCount(&self) -> i32 {
        self.follower_count
    }

    // TODO: components
    // TODO: homeComponent

    async fn featured(&self) -> bool {
        self.featured
    }

    async fn verified(&self) -> bool {
        self.verified
    }

    async fn partnered(&self) -> bool {
        self.partnered
    }

    // TODO: members

    async fn featuredDescription(&self) -> String {
        self.featured_description.clone()
    }

    // TODO: invites

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

    async fn css(&self) -> String {
        self.css.clone()
    }

    async fn description(&self) -> Option<String> {
        self.description.clone()
    }

    // TODO: customEmojis
    // TODO: unread
    // TODO: mentioned
}
