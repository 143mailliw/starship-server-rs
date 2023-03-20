use crate::entities::prelude::User;
use crate::entities::user;
use crate::errors;
use crate::guards::session::{SessionGuard, SessionType};
use crate::sessions::Session;
use async_graphql::{Context, Description, Error, Object, ID};
use log::error;
use sea_orm::{DatabaseConnection, EntityTrait, PaginatorTrait, QueryOrder};

#[derive(Default, Description)]
pub struct UserQuery;

#[Object(rename_fields = "camelCase", rename_args = "camelCase")]
impl UserQuery {
    /// Finds a user from it's ID.
    #[graphql(complexity = 5)]
    async fn user(&self, ctx: &Context<'_>, id: ID) -> Result<user::Model, Error> {
        let db = ctx.data::<DatabaseConnection>().unwrap();

        User::find_by_id(id.to_string())
            .one(db)
            .await
            .map_err(|_| errors::create_internal_server_error(None, "RETRIEVAL_ERROR"))?
            .ok_or(errors::create_not_found_error())
    }

    /// Retrieves the current session's user, if it have one.
    #[graphql(guard = "SessionGuard::new(SessionType::User)", complexity = 10)]
    async fn current_user(&self, ctx: &Context<'_>) -> Result<user::Model, Error> {
        Ok(ctx.data::<Session>().unwrap().user.clone().unwrap())
    }

    /// Retrieves many users, for use in the system administration dashboard.
    #[graphql(
        guard = "SessionGuard::new(SessionType::Admin)",
        complexity = "5 * size as usize + size as usize * child_complexity"
    )]
    async fn admin_users(
        &self,
        ctx: &Context<'_>,
        size: u64,
        page: u64,
    ) -> Result<Vec<user::Model>, Error> {
        let db = ctx.data::<DatabaseConnection>().unwrap();

        User::find()
            .order_by_desc(user::Column::Created)
            .paginate(db, size)
            .fetch_page(page)
            .await
            .map_err(|_| errors::create_internal_server_error(None, "RETRIEVAL_ERROR"))
    }
}
