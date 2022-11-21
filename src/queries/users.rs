#![allow(non_snake_case)]
use crate::entities::prelude::User;
use crate::entities::user;
use crate::errors;
use crate::guards::session::{SessionGuard, SessionType};
use crate::sessions::Session;
use async_graphql::{Context, Error, Object, ID};
use log::error;
use sea_orm::{DatabaseConnection, EntityTrait};

#[Object]
impl super::Query {
    async fn user(&self, ctx: &Context<'_>, id: ID) -> Result<user::Model, Error> {
        let db = ctx.data::<DatabaseConnection>().unwrap();

        match User::find_by_id(id.to_string()).one(db).await {
            Ok(value) => match value {
                Some(value) => Ok(value),
                None => Err(errors::create_internal_server_error(None, "BAD_ID_ERROR")),
            },
            Err(error) => {
                error!("{}", error);
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
}
