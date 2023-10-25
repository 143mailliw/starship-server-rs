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
    #[graphql(guard = "SessionGuard::new(SessionType::User)", complexity = 200)]
    async fn leave_planet(&self, ctx: &Context<'_>, id: ID) -> Result<bool, Error> {
        let db = ctx.data::<DatabaseConnection>().unwrap();
        let session = ctx.data::<Session>().unwrap();
        let user_id = session.user.as_ref().map(|user| user.id.clone());

        let planet = planet::Entity::find_by_id(id.to_string())
            .one(db)
            .await
            .map_err(|_| errors::create_internal_server_error(None, "PLANET_RETRIEVAL_ERROR"))?
            .ok_or(errors::create_user_input_error(
                "You aren't a member of that planet.",
                "NOT_MEMBER",
            ))?;

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

        if member.banned {
            return Err(errors::create_user_input_error(
                "You cannot leave a planet you are banned from",
                "BANNED",
            ));
        }

        member
            .delete(db)
            .await
            .map_err(|_| errors::create_internal_server_error(None, "MEMBER_DELETION_ERROR"))?;

        Ok(true)
    }

    /// Updates permissions for the specified planet member.
    ///
    /// + grants the permission
    /// * falls back to the previous permission set (for this function, the highest priority role)
    /// - explicitly denies the permission
    #[graphql(complexity = 50)]
    async fn update_member_permissions(
        &self,
        ctx: &Context<'_>,
        id: ID,
        permissions: Vec<String>,
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

        let mut active_member: planet_member::ActiveModel = member.clone().into();
        active_member.permissions =
            ActiveValue::Set(util::update_permissions(member.permissions, permissions)?);

        active_member
            .update(db)
            .await
            .map_err(|_| errors::create_internal_server_error(None, "UPDATE_ERROR"))
    }

    /// Kicks a member from a planet.
    #[graphql(complexity = 50)]
    async fn kick_member(&self, ctx: &Context<'_>, id: ID) -> Result<bool, Error> {
        let db = ctx.data::<DatabaseConnection>().unwrap();
        let session = ctx.data::<Session>().unwrap();
        let user_id = session.user.as_ref().map(|user| user.id.clone());

        let id = id.to_string();

        let kick_member = planet_member::Entity::find_by_id(id.clone())
            .one(db)
            .await
            .map_err(|_| errors::create_internal_server_error(None, "MEMBER_RETRIEVAL_ERROR"))?
            .ok_or(errors::create_not_found_error())?;

        let planet = util::get_planet(kick_member.planet.clone(), db).await?;
        let member =
            util::get_planet_member(user_id.clone(), kick_member.planet.clone(), db).await?;
        let roles = util::get_member_roles(member.clone(), db).await?;

        util::check_permission("planet.member.kick", &planet, member, roles)?;

        if kick_member.user == user_id.unwrap() {
            return Err(errors::create_user_input_error(
                "You cannot kick yourself.",
                "SELF",
            ));
        }

        if kick_member.user == planet.owner {
            return Err(errors::create_user_input_error(
                "You cannot kick the owner of the planet.",
                "PLANET_OWNER",
            ));
        }

        if kick_member.banned {
            return Err(errors::create_user_input_error(
                "You can not kick a banned member.",
                "BANNED",
            ));
        }

        kick_member
            .delete(db)
            .await
            .map_err(|_| errors::create_internal_server_error(None, "MEMBER_DELETION_ERROR"))?;

        Ok(true)
    }

    /// Toggles whether or not a member is banned.
    #[graphql(complexity = 50)]
    async fn ban_member(&self, ctx: &Context<'_>, id: ID) -> Result<planet_member::Model, Error> {
        let db = ctx.data::<DatabaseConnection>().unwrap();
        let session = ctx.data::<Session>().unwrap();
        let user_id = session.user.as_ref().map(|user| user.id.clone());

        let id = id.to_string();

        let ban_member = planet_member::Entity::find_by_id(id.clone())
            .one(db)
            .await
            .map_err(|_| errors::create_internal_server_error(None, "MEMBER_RETRIEVAL_ERROR"))?
            .ok_or(errors::create_not_found_error())?;

        let planet = util::get_planet(ban_member.planet.clone(), db).await?;
        let member =
            util::get_planet_member(user_id.clone(), ban_member.planet.clone(), db).await?;
        let roles = util::get_member_roles(member.clone(), db).await?;

        util::check_permission("planet.member.kick", &planet, member, roles)?;

        if ban_member.user == user_id.unwrap() {
            return Err(errors::create_user_input_error(
                "You cannot ban yourself.",
                "SELF",
            ));
        }

        if ban_member.user == planet.owner {
            return Err(errors::create_user_input_error(
                "You cannot ban the owner of the planet.",
                "PLANET_OWNER",
            ));
        }

        let banned = !ban_member.banned;

        let mut active_member: planet_member::ActiveModel = ban_member.clone().into();
        active_member.banned = ActiveValue::Set(banned);

        active_member
            .update(db)
            .await
            .map_err(|_| errors::create_internal_server_error(None, "UPDATE_ERROR"))
    }

    /// Adds a role to the planet member.
    #[graphql(complexity = 50)]
    async fn add_role_member(
        &self,
        ctx: &Context<'_>,
        member_id: ID,
        role_id: ID,
    ) -> Result<planet_member::Model, Error> {
        let db = ctx.data::<DatabaseConnection>().unwrap();
        let session = ctx.data::<Session>().unwrap();
        let user_id = session.user.as_ref().map(|user| user.id.clone());

        let target_role = planet_role::Entity::find_by_id(role_id.to_string())
            .one(db)
            .await
            .map_err(|_| errors::create_internal_server_error(None, "TARGET_ROLE_RETRIEVAL_ERROR"))?
            .ok_or(errors::create_not_found_error())?;

        let planet = util::get_planet(target_role.planet.clone(), db).await?;
        let member =
            util::get_planet_member(user_id.clone(), target_role.planet.clone(), db).await?;
        let roles = util::get_member_roles(member.clone(), db).await?;
        util::check_permission(
            "planet.roles.add_member",
            &planet,
            member.clone(),
            roles.clone(),
        )?;
        util::high_enough(roles, vec![target_role.clone()], member)?;

        let target_member = planet_member::Entity::find_by_id(member_id.to_string())
            .one(db)
            .await
            .map_err(|_| {
                errors::create_internal_server_error(None, "TARGET_MEMBER_RETRIEVAL_ERROR")
            })?
            .ok_or(errors::create_not_found_error())?;

        if target_member.planet != target_role.planet {
            return Err(errors::create_not_found_error());
        }

        if target_member.roles.contains(&target_role.id) {
            return Err(errors::create_user_input_error(
                "This user already has that role.",
                "ALREADY_HAS_ROLE",
            ));
        }

        let mut new_roles = target_member.roles.clone();
        new_roles.push(target_role.id);

        let mut active_member: planet_member::ActiveModel = target_member.clone().into();
        active_member.roles = ActiveValue::Set(new_roles);

        active_member
            .update(db)
            .await
            .map_err(|_| errors::create_internal_server_error(None, "UPDATE_ERROR"))
    }

    /// Removes a role from a planet member.
    #[graphql(complexity = 50)]
    async fn remove_role_member(
        &self,
        ctx: &Context<'_>,
        member_id: ID,
        role_id: ID,
    ) -> Result<planet_member::Model, Error> {
        let db = ctx.data::<DatabaseConnection>().unwrap();
        let session = ctx.data::<Session>().unwrap();
        let user_id = session.user.as_ref().map(|user| user.id.clone());

        let target_role = planet_role::Entity::find_by_id(role_id.to_string())
            .one(db)
            .await
            .map_err(|_| errors::create_internal_server_error(None, "TARGET_ROLE_RETRIEVAL_ERROR"))?
            .ok_or(errors::create_not_found_error())?;

        let planet = util::get_planet(target_role.planet.clone(), db).await?;
        let member =
            util::get_planet_member(user_id.clone(), target_role.planet.clone(), db).await?;
        let roles = util::get_member_roles(member.clone(), db).await?;
        util::check_permission(
            "planet.roles.remove_member",
            &planet,
            member.clone(),
            roles.clone(),
        )?;
        util::high_enough(roles, vec![target_role.clone()], member)?;

        let target_member = planet_member::Entity::find_by_id(member_id.to_string())
            .one(db)
            .await
            .map_err(|_| {
                errors::create_internal_server_error(None, "TARGET_MEMBER_RETRIEVAL_ERROR")
            })?
            .ok_or(errors::create_not_found_error())?;

        if target_member.planet != target_role.planet {
            return Err(errors::create_not_found_error());
        }

        if !target_member.roles.contains(&target_role.id) {
            return Err(errors::create_user_input_error(
                "This user doesn't have that role",
                "ALREADY_MISSING_ROLE",
            ));
        }

        let mut new_roles = target_member.roles.clone();
        new_roles.retain(|v| *v != target_role.id);

        let mut active_member: planet_member::ActiveModel = target_member.clone().into();
        active_member.roles = ActiveValue::Set(new_roles);

        active_member
            .update(db)
            .await
            .map_err(|_| errors::create_internal_server_error(None, "UPDATE_ERROR"))
    }
}
