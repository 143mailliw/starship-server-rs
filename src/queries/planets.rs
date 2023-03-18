use crate::entities::planet;
use crate::errors;
use crate::permissions::util;
use crate::sessions::Session;
use async_graphql::{Context, Description, Error, Object, ID};
use log::error;
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};

#[derive(Default, Description)]
pub struct PlanetQuery;

#[Object(rename_fields = "camelCase", rename_args = "camelCase")]
impl PlanetQuery {
    /// Finds a planet from it's ID.
    #[graphql(complexity = 5)]
    async fn planet(&self, ctx: &Context<'_>, id: ID) -> Result<planet::Model, Error> {
        let db = ctx.data::<DatabaseConnection>().unwrap();
        let session = ctx.data::<Session>().unwrap();

        match planet::Entity::find_by_id(id.to_string()).one(db).await {
            Ok(value) => match value {
                Some(value) => {
                    let user_id = session.user.as_ref().map(|user| user.id.clone());
                    let member = util::get_planet_member(user_id, id.to_string(), db).await?;
                    let roles = util::get_member_roles(member.clone(), db).await?;
                    util::check_permission(
                        "planet.view".to_string(),
                        value.clone(),
                        member,
                        roles,
                    )?;
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

    /// Finds all the featured planets.
    #[graphql(complexity = 5)]
    async fn featured_planets(&self, ctx: &Context<'_>) -> Result<Vec<planet::Model>, Error> {
        let db = ctx.data::<DatabaseConnection>().unwrap();

        match planet::Entity::find()
            .filter(planet::Column::Featured.eq(true))
            .all(db)
            .await
        {
            Ok(value) => Ok(value),
            Err(_err) => Err(errors::create_internal_server_error(
                None,
                "RETRIEVAL_ERROR",
            )),
        }
    }
}
