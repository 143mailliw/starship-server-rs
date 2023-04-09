use crate::entities::planet_role;
use crate::errors;
use crate::permissions::util;
use crate::sessions::Session;
use async_graphql::{Context, Description, Error, Object, ID};
use nanoid::nanoid;
use sea_orm::{ActiveValue, DatabaseConnection, EntityTrait, QueryOrder, TryIntoModel};

#[derive(Default, Description)]
pub struct RoleMutation;

#[Object(rename_fields = "camelCase", rename_args = "camelCase")]
impl RoleMutation {
    /// Creates a new role.
    #[graphql(complexity = 50)]
    async fn insert_role(
        &self,
        ctx: &Context<'_>,
        planet_id: ID,
        name: String,
    ) -> Result<planet_role::Model, Error> {
        let db = ctx.data::<DatabaseConnection>().unwrap();
        let session = ctx.data::<Session>().unwrap();
        let user_id = session.user.as_ref().map(|user| user.id.clone());

        let planet = util::get_planet(planet_id.to_string(), db).await?;
        let member = util::get_planet_member(user_id, planet_id.to_string(), db).await?;
        let roles = util::get_member_roles(member.clone(), db).await?;
        util::check_permission("planet.roles.create", &planet, member, roles)?;

        let position = planet_role::Entity::find()
            .order_by_desc(planet_role::Column::Position)
            .one(db)
            .await
            .map_err(|_| errors::create_internal_server_error(None, "ORDER_RETRIEVAL_ERROR"))?
            .map_or(0, |role| role.position)
            + 1;

        let new_role = planet_role::ActiveModel {
            id: ActiveValue::Set(nanoid!(16)),
            name: ActiveValue::Set(name),
            color: ActiveValue::Set("#FFFFFF".to_string()),
            permissions: ActiveValue::Set(vec![]),
            default: ActiveValue::Set(false),
            planet: ActiveValue::Set(planet_id.to_string()),
            position: ActiveValue::Set(position),
        };

        planet_role::Entity::insert(new_role.clone())
            .exec(db)
            .await
            .map_err(|_| errors::create_internal_server_error(None, "INSERTION_ERROR"))?;

        new_role
            .try_into_model()
            .map_err(|_| errors::create_internal_server_error(None, "CONVERSION_ERROR"))
    }
}
