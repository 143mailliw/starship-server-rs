use crate::entities::{planet, planet_member, planet_role};
use crate::errors;
use crate::guards::session::{SessionGuard, SessionType};
use crate::permissions::util;
use crate::sessions::Session;
use async_graphql::{Context, Description, Error, Object, ID};
use nanoid::nanoid;
use sea_orm::{
    ActiveModelTrait, ActiveValue, ColumnTrait, DatabaseConnection, EntityTrait, ModelTrait,
    QueryFilter,
};

#[derive(Default, Description)]
pub struct MemberMutation;

#[Object(rename_fields = "camelCase", rename_args = "camelCase")]
impl MemberMutation {
    /// Joins a public planet.
    #[graphql(guard = "SessionGuard::new(SessionType::User)", complexity = 200)]
    async fn join_planet(&self, ctx: &Context<'_>, id: ID) -> Result<planet_member::Model, Error> {
        let db = ctx.data::<DatabaseConnection>().unwrap();
        let session = ctx.data::<Session>().unwrap();
        let user_id = session.user.as_ref().map(|user| user.id.clone());

        let planet = planet::Entity::find_by_id(id.to_string())
            .one(db)
            .await
            .map_err(|_| errors::create_internal_server_error(None, "PLANET_RETRIEVAL_ERROR"))?
            .ok_or(errors::create_not_found_error())?;

        let role = planet_role::Entity::find()
            .filter(
                planet_role::Column::Planet
                    .eq(planet.id.clone())
                    .and(planet_role::Column::Default.eq(true)),
            )
            .one(db)
            .await
            .map_err(|_| errors::create_internal_server_error(None, "ROLE_RETRIEVAL_ERROR"))?
            .ok_or(errors::create_internal_server_error(
                None,
                "MISSING_DEFAULT_ROLE_ERROR",
            ))?;

        if planet.private {
            Err(errors::create_not_found_error())
        } else {
            if planet_member::Entity::find()
                .filter(
                    planet_member::Column::User
                        .eq(user_id.as_ref().unwrap().clone())
                        .and(planet_member::Column::Planet.eq(planet.id.clone())),
                )
                .one(db)
                .await
                .map_err(|_| {
                    errors::create_internal_server_error(None, "MEMBER_CHECK_RETRIEVAL_ERROR")
                })?
                .is_some()
            {
                return Err(errors::create_user_input_error(
                    "You are already a member of this planet.",
                    "ALREADY_MEMBER",
                ));
            }

            let member = planet_member::ActiveModel {
                id: ActiveValue::Set(nanoid!(16)),
                planet: ActiveValue::Set(planet.id),
                user: ActiveValue::Set(user_id.unwrap()),
                roles: ActiveValue::Set(vec![role.id]),
                permissions: ActiveValue::Set(vec![]),
                created: ActiveValue::Set(chrono::offset::Utc::now().naive_utc()),
                banned: ActiveValue::Set(false),
            };

            let insertion = planet_member::Entity::insert(member)
                .exec(db)
                .await
                .map_err(|_| errors::create_internal_server_error(None, "INSERTION_ERROR"))?;

            Ok(planet_member::Entity::find_by_id(insertion.last_insert_id)
                .one(db)
                .await
                .map_err(|_| errors::create_internal_server_error(None, "MEMBER_RETRIEVAL_ERROR"))?
                .ok_or(errors::create_internal_server_error(None, "FIND_ERROR"))?)
        }
    }

    /// Leaves a planet the current user is a member of.
    async fn leave_planet(&self, ctx: &Context<'_>, id: ID) -> Result<bool, Error> {
        let db = ctx.data::<DatabaseConnection>().unwrap();
        let session = ctx.data::<Session>().unwrap();
        let user_id = session.user.as_ref().map(|user| user.id.clone());

        let planet = planet::Entity::find_by_id(id.to_string())
            .one(db)
            .await
            .map_err(|_| errors::create_internal_server_error(None, "PLANET_RETRIEVAL_ERROR"))?
            .ok_or(errors::create_not_found_error())?;

        let user_id = user_id.as_ref().unwrap().clone();

        if user_id == planet.owner {
            return Err(errors::create_user_input_error(
                "You cannot leave a planet you are the owner of.",
                "PLANET_OWNER",
            ));
        }

        let member = planet_member::Entity::find()
            .filter(
                planet_member::Column::User
                    .eq(user_id)
                    .and(planet_member::Column::Planet.eq(planet.id.clone())),
            )
            .one(db)
            .await
            .map_err(|_| errors::create_internal_server_error(None, "MEMBER_RETRIEVAL_ERROR"))?
            .ok_or(errors::create_user_input_error(
                "You aren't a member of that planet.",
                "NOT_MEMBER",
            ))?;

        member
            .delete(db)
            .await
            .map_err(|_| errors::create_internal_server_error(None, "MEMBER_DELETION_ERROR"))?;

        Ok(true)
    }

    /// Sets a permission for the specified planet member.
    ///
    /// + grants the permission
    /// * falls back to the previous permission set (for this function, the highest priority role)
    /// - explicitly denies the permission
    async fn set_member_permission(
        &self,
        ctx: &Context<'_>,
        id: ID,
        permission: String,
    ) -> Result<planet_member::Model, Error> {
        let db = ctx.data::<DatabaseConnection>().unwrap();
        let session = ctx.data::<Session>().unwrap();
        let user_id = session.user.as_ref().map(|user| user.id.clone());

        let member = planet_member::Entity::find_by_id(id.to_string())
            .one(db)
            .await
            .map_err(|_| errors::create_internal_server_error(None, "TARGET_RETRIEVAL_ERROR"))?
            .ok_or(errors::create_not_found_error())?;

        let planet = util::get_planet(member.planet.clone().to_string(), db).await?;
        let requesting_member =
            util::get_planet_member(user_id, member.planet.clone().to_string(), db).await?;

        let roles = util::get_member_roles(requesting_member.clone(), db).await?;
        util::check_permission(
            "planet.member.edit_permissions",
            &planet,
            requesting_member,
            roles,
        )?;

        if permission.ends_with("owner") {
            return Err(errors::create_user_input_error(
                "You cannot grant or remove the 'owner' permission.",
                "SPECIAL_PERMISSION",
            ));
        }

        let mut active_member: planet_member::ActiveModel = member.clone().into();
        active_member.permissions =
            ActiveValue::Set(util::update_permissions(member.permissions, permission));

        active_member
            .update(db)
            .await
            .map_err(|_| errors::create_internal_server_error(None, "UPDATE_ERROR"))
    }
}
