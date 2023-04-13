use crate::entities::planet_role;
use crate::errors;
use crate::permissions::util;
use crate::sessions::Session;
use async_graphql::{Context, Description, Error, Object, ID};
use sea_orm::{DatabaseConnection, EntityTrait};

#[derive(Default, Description)]
pub struct RoleQuery;

#[Object(rename_fields = "camelCase", rename_args = "camelCase")]
impl RoleQuery {
    /// Finds a role from it's ID.
    #[graphql(complexity = 5)]
    async fn role(&self, ctx: &Context<'_>, id: ID) -> Result<planet_role::Model, Error> {
        let db = ctx.data::<DatabaseConnection>().unwrap();
        let session = ctx.data::<Session>().unwrap();
        let user_id = session.user.as_ref().map(|user| user.id.clone());

        let queried_role = planet_role::Entity::find_by_id(id.to_string())
            .one(db)
            .await
            .map_err(|_| errors::create_forbidden_error(None, "RETRIEVAL_ERROR"))?
            .ok_or(errors::create_not_found_error())?;

        let planet = util::get_planet(queried_role.planet.clone(), db).await?;
        let member = util::get_planet_member(user_id.clone(), id.to_string(), db).await?;
        let roles = util::get_member_roles(member.clone(), db).await?;
        util::check_permission("planet.view", &planet, member, roles)?;

        Ok(queried_role)
    }
}
