use crate::entities::planet_member;
use crate::errors;
use crate::permissions::util;
use crate::sessions::Session;
use async_graphql::{Context, Description, Error, Object, ID};
use log::error;
use sea_orm::{DatabaseConnection, EntityTrait};

#[derive(Default, Description)]
pub struct MemberQuery;

#[Object(rename_fields = "camelCase", rename_args = "camelCase")]
impl MemberQuery {
    /// Finds a planet from it's ID.
    #[graphql(complexity = 5)]
    async fn planet_member(
        &self,
        ctx: &Context<'_>,
        id: ID,
    ) -> Result<planet_member::Model, Error> {
        let db = ctx.data::<DatabaseConnection>().unwrap();
        let session = ctx.data::<Session>().unwrap();
        let user_id = session.user.as_ref().map(|user| user.id.clone());

        match planet_member::Entity::find_by_id(id.to_string())
            .one(db)
            .await
        {
            Ok(value) => match value {
                Some(value) => {
                    let planet = util::get_planet(value.planet.clone(), db).await?;
                    let member =
                        util::get_planet_member(user_id.clone(), id.to_string(), db).await?;
                    let roles = util::get_member_roles(member.clone(), db).await?;

                    // users always need to be allowed to view their own members or else the client
                    // will be unable to determine permissions
                    if user_id != Some(value.user.clone()) {
                        util::check_permission(
                            "planet.member.view".to_string(),
                            planet,
                            member,
                            roles,
                        )?;
                    }

                    Ok(value)
                }
                None => Err(errors::create_not_found_error()),
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
}
