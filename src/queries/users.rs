#![allow(non_snake_case)]
use crate::entities::prelude::User;
use crate::entities::user;
use crate::errors;
use crate::guards::session::{SessionGuard, SessionType};
use crate::sessions::Session;
use async_graphql::{Context, Error, Object, ID};
use log::error;
use sea_orm::{DatabaseConnection, EntityTrait, PaginatorTrait, QueryOrder};

#[derive(Default)]
pub struct UserQuery;

#[Object]
impl UserQuery {
    #[graphql(complexity = 5)] //just ensure that they can't do it multiple times
    async fn user(&self, ctx: &Context<'_>, id: ID) -> Result<user::Model, Error> {
        let db = ctx.data::<DatabaseConnection>().unwrap();

        match User::find_by_id(id.to_string()).one(db).await {
            Ok(value) => match value {
                Some(value) => Ok(value),
                None => Err(errors::create_user_input_error(
                    "User does not exist.",
                    "INVALID_USER",
                )),
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

    #[graphql(guard = "SessionGuard::new(SessionType::User)", complexity = 10)]
    async fn currentUser(&self, ctx: &Context<'_>) -> Result<user::Model, Error> {
        Ok(ctx.data::<Session>().unwrap().user.clone().unwrap())
    }

    #[graphql(
        guard = "SessionGuard::new(SessionType::Admin)",
        complexity = "5 * size as usize + size as usize * child_complexity"
    )]
    async fn adminUsers(
        &self,
        ctx: &Context<'_>,
        size: u64,
        page: u64,
    ) -> Result<Vec<user::Model>, Error> {
        let db = ctx.data::<DatabaseConnection>().unwrap();

        match User::find()
            .order_by_desc(user::Column::Created)
            .paginate(db, size)
            .fetch_page(page)
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
