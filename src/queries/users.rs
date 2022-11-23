#![allow(non_snake_case)]
use crate::entities::prelude::User;
use crate::entities::user;
use crate::errors;
use crate::guards::session::{SessionGuard, SessionType};
use crate::sessions::Session;
use async_graphql::{Context, Error, Object, ID};
use chrono::NaiveDateTime;
use log::error;
use sea_orm::{CursorTrait, DatabaseConnection, EntityTrait, QueryOrder};

#[derive(Default)]
pub struct UserQuery;

#[Object]
impl UserQuery {
    async fn user(&self, ctx: &Context<'_>, id: ID) -> Result<user::Model, Error> {
        let db = ctx.data::<DatabaseConnection>().unwrap();

        match User::find_by_id(id.to_string()).one(db).await {
            Ok(value) => match value {
                Some(value) => Ok(value),
                None => Err(errors::create_internal_server_error(None, "BAD_ID_ERROR")),
            },
            Err(error) => {
                error!("{error}");
                Err(errors::create_internal_server_error(
                    None,
                    "RETRIEVAL_ERROR",
                ))
            }
        }
    }

    #[graphql(guard = "SessionGuard::new(SessionType::User)")]
    async fn currentUser(&self, ctx: &Context<'_>) -> Result<user::Model, Error> {
        Ok(ctx.data::<Session>().unwrap().user.clone().unwrap())
    }

    #[graphql(
        guard = "SessionGuard::new(SessionType::Admin)",
        deprecation = "use user, adminUser is redundant"
    )]
    async fn adminUser(&self, ctx: &Context<'_>, id: ID) -> Result<user::Model, Error> {
        self.user(ctx, id).await
    }

    #[graphql(guard = "SessionGuard::new(SessionType::Admin)")]
    async fn adminUsers(
        &self,
        ctx: &Context<'_>,
        limit: u64,
        cursor: NaiveDateTime,
    ) -> Result<Vec<user::Model>, Error> {
        let db = ctx.data::<DatabaseConnection>().unwrap();

        match User::find()
            .cursor_by(user::Column::Created)
            .after(cursor)
            .last(limit)
            .all(db)
            .await
        {
            Ok(values) => Ok(values),
            Err(error) => {
                error!("{error}");
                Err(errors::create_internal_server_error(
                    None,
                    "RETRIEVAL_ERROR",
                ))
            }
        }
    }
}
