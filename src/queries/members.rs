use crate::entities::planet_member;
use crate::errors;
use crate::permissions::util;
use crate::sessions::Session;
use async_graphql::{Context, Description, Error, Object, ID};
use sea_orm::{DatabaseConnection, EntityTrait};

#[derive(Default, Description)]
pub struct MemberQuery;

#[Object(rename_fields = "camelCase", rename_args = "camelCase")]
impl MemberQuery {
    /// Finds a planet member from their ID.
    #[graphql(complexity = 5)]
    async fn planet_member(
        &self,
        ctx: &Context<'_>,
        id: ID,
    ) -> Result<planet_member::Model, Error> {
        let db = ctx.data::<DatabaseConnection>().unwrap();
        let session = ctx.data::<Session>().unwrap();
        let user_id = session.user.as_ref().map(|user| user.id.clone());

        let queried_member = planet_member::Entity::find_by_id(id.to_string())
            .one(db)
            .await
            .map_err(|_| errors::create_forbidden_error(None, "RETRIEVAL_ERROR"))?
            .ok_or(errors::create_not_found_error())?;

        let planet = util::get_planet(queried_member.planet.clone(), db).await?;
        let member = util::get_planet_member(user_id.clone(), id.to_string(), db).await?;
        let roles = util::get_member_roles(member.clone(), db).await?;

        // users always need to be allowed to view their own members or else the client
        // will be unable to determine permissions
        if user_id != Some(queried_member.user.clone()) {
            util::check_permission("planet.member.view", &planet, member, roles)?;
        }

        Ok(queried_member)
    }
}
