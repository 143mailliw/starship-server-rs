use crate::entities::planet_role;
use crate::errors;
use crate::permissions::util;
use crate::sessions::Session;
use async_graphql::{Context, Description, Error, InputObject, Object, ID};
use nanoid::nanoid;
use sea_orm::{
    ActiveModelTrait, ActiveValue, ColumnTrait, ConnectionTrait, DatabaseBackend,
    DatabaseConnection, EntityTrait, QueryFilter, QueryOrder, Statement, TransactionTrait,
    TryIntoModel,
};

#[derive(InputObject, Debug, Clone)]
pub struct RoleOrderEntry {
    role: String,
    position: i32,
}

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
            .order_by_asc(planet_role::Column::Position)
            .one(db)
            .await
            .map_err(|_| errors::create_internal_server_error(None, "ORDER_RETRIEVAL_ERROR"))?
            .map_or(0, |role| role.position)
            - 1;

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

    /// Renames and changes the color of a role.
    #[graphql(complexity = 50)]
    async fn update_role(
        &self,
        ctx: &Context<'_>,
        id: ID,
        name: String,
        color: String,
    ) -> Result<planet_role::Model, Error> {
        let db = ctx.data::<DatabaseConnection>().unwrap();
        let session = ctx.data::<Session>().unwrap();
        let user_id = session.user.as_ref().map(|user| user.id.clone());

        let role = planet_role::Entity::find_by_id(id.to_string())
            .one(db)
            .await
            .map_err(|_| errors::create_internal_server_error(None, "ROLE_RETRIEVAL_ERROR"))?
            .ok_or(errors::create_not_found_error())?;

        let planet = util::get_planet(role.planet.clone(), db).await?;
        let member = util::get_planet_member(user_id, role.planet.clone(), db).await?;
        let roles = util::get_member_roles(member.clone(), db).await?;
        util::check_permission("planet.roles.edit", &planet, member.clone(), roles.clone())?;
        util::high_enough(roles, vec![role.clone()], member)?;

        if color.len() != 7 && color.len() != 9 {
            return Err(errors::create_user_input_error(
                "Invalid color code.",
                "INVALID_COLOR",
            ));
        }

        let mut active_role: planet_role::ActiveModel = role.into();
        active_role.name = ActiveValue::Set(name);
        active_role.color = ActiveValue::Set(color);

        active_role
            .update(db)
            .await
            .map_err(|_| errors::create_internal_server_error(None, "UPDATE_ERROR"))
    }

    /// Updates permissions for the specified planet role.
    ///
    /// + grants the permission
    /// * falls back to the previous permission set (for this function, the highest priority role)
    /// - explicitly denies the permission
    #[graphql(complexity = 50)]
    async fn update_role_permissions(
        &self,
        ctx: &Context<'_>,
        id: ID,
        permissions: Vec<String>,
    ) -> Result<planet_role::Model, Error> {
        let db = ctx.data::<DatabaseConnection>().unwrap();
        let session = ctx.data::<Session>().unwrap();
        let user_id = session.user.as_ref().map(|user| user.id.clone());

        let role = planet_role::Entity::find_by_id(id.to_string())
            .one(db)
            .await
            .map_err(|_| errors::create_internal_server_error(None, "ROLE_RETRIEVAL_ERROR"))?
            .ok_or(errors::create_not_found_error())?;

        let planet = util::get_planet(role.planet.clone(), db).await?;
        let member = util::get_planet_member(user_id, role.planet.clone(), db).await?;
        let roles = util::get_member_roles(member.clone(), db).await?;
        util::check_permission(
            "planet.roles.edit_permissions",
            &planet,
            member.clone(),
            roles.clone(),
        )?;
        util::high_enough(roles, vec![role.clone()], member)?;

        let mut active_role: planet_role::ActiveModel = role.clone().into();
        active_role.permissions =
            ActiveValue::Set(util::update_permissions(role.permissions, permissions)?);

        active_role
            .update(db)
            .await
            .map_err(|_| errors::create_internal_server_error(None, "UPDATE_ERROR"))
    }

    /// Deletes a role.
    #[graphql(complexity = 500)]
    async fn delete_role(&self, ctx: &Context<'_>, id: ID) -> Result<bool, Error> {
        let db = ctx.data::<DatabaseConnection>().unwrap();
        let session = ctx.data::<Session>().unwrap();
        let user_id = session.user.as_ref().map(|user| user.id.clone());

        let role = planet_role::Entity::find_by_id(id.to_string())
            .one(db)
            .await
            .map_err(|_| errors::create_internal_server_error(None, "ROLE_RETRIEVAL_ERROR"))?
            .ok_or(errors::create_not_found_error())?;

        let planet = util::get_planet(role.planet.clone(), db).await?;
        let member = util::get_planet_member(user_id, role.planet.clone(), db).await?;
        let roles = util::get_member_roles(member.clone(), db).await?;
        util::check_permission(
            "planet.roles.delete",
            &planet,
            member.clone(),
            roles.clone(),
        )?;
        util::high_enough(roles, vec![role.clone()], member)?;

        db.query_one(Statement::from_sql_and_values(
            DatabaseBackend::Postgres,
            r#"SELECT ARRAY_REMOVE(roles, $1) FROM planet_member;"#,
            [role.id.clone().into()],
        ))
        .await
        .map_err(|_| errors::create_internal_server_error(None, "REMOVE_ARRAY_ERROR"))?;

        let active_role: planet_role::ActiveModel = role.into();

        active_role
            .delete(db)
            .await
            .map_err(|_| errors::create_internal_server_error(None, "DELETE_ROLE_ERROR"))
            .map(|_| true)
    }

    /// Changes the order of roles.
    #[graphql(complexity = 200)]
    async fn reorder_roles(
        &self,
        ctx: &Context<'_>,
        planet_id: ID,
        mut positions: Vec<RoleOrderEntry>,
    ) -> Result<Vec<planet_role::Model>, Error> {
        let db = ctx.data::<DatabaseConnection>().unwrap();
        let session = ctx.data::<Session>().unwrap();
        let user_id = session.user.as_ref().map(|user| user.id.clone());

        if positions.is_empty() {
            return Err(errors::create_user_input_error(
                "No roles were moved.",
                "NO_ORDER_CHANGES",
            ));
        }

        // get all of the roles for the planet and filter down to the ones we need
        let target_roles = planet_role::Entity::find()
            .filter(planet_role::Column::Planet.eq(planet_id.to_string()))
            .all(db)
            .await
            .map_err(|_| errors::create_internal_server_error(None, "ROLES_RETRIEVAL_ERROR"))?;

        let filtered_roles: Vec<planet_role::Model> = target_roles
            .iter()
            .filter(|r| positions.iter().any(|p| p.role == r.id))
            .cloned()
            .collect();

        if filtered_roles.len() != positions.len() {
            return Err(errors::create_not_found_error());
        }

        // ensure we even have permission to do this
        let planet = util::get_planet(planet_id.to_string(), db).await?;
        let member = util::get_planet_member(user_id, planet_id.to_string(), db).await?;
        let roles = util::get_member_roles(member.clone(), db).await?;
        util::check_permission(
            "planet.roles.delete",
            &planet,
            member.clone(),
            roles.clone(),
        )?;
        util::high_enough(roles.clone(), target_roles.clone(), member)?;

        // check that we're not moving to a position higher than our member's highest role
        positions.sort_by_key(|change| change.position);
        let highest_position = positions.last().map_or(i32::MIN, |change| change.position);

        let mut member_roles = roles.ok_or(errors::create_not_found_error())?;
        member_roles.sort_by_key(|role| role.position);
        let highest_held_position = member_roles.last().map_or(i32::MIN, |role| role.position);

        if highest_position >= highest_held_position {
            return Err(errors::create_forbidden_error(
                Some("You can't move a role that high."),
                "POSITION_TOO_HIGH",
            ));
        }

        // check for duplicate positions
        let mut dupe_check_vec: Vec<RoleOrderEntry> = target_roles
            .iter()
            .map(|r| RoleOrderEntry {
                role: r.id.clone(),
                position: r.position,
            })
            .filter(|r| !positions.iter().any(|p| p.role == r.role))
            .chain(positions.iter().cloned())
            .collect();

        dupe_check_vec.sort_by_key(|r| r.position);
        dupe_check_vec.dedup_by_key(|r| r.position);

        if dupe_check_vec.len() != target_roles.len() {
            return Err(errors::create_user_input_error(
                "Two roles cannot occupy the same position.",
                "DUPLICATE_POSITION",
            ));
        }

        // store the updated roles
        let txn = db.begin().await?;

        for role in filtered_roles {
            let mut active_role: planet_role::ActiveModel = role.clone().into();
            active_role.position = ActiveValue::Set(
                positions
                    .iter()
                    .find(|p| p.role == role.id)
                    .unwrap()
                    .position,
            );
            active_role.update(&txn).await?;
        }

        txn.commit().await?;

        // return the updated roles
        planet_role::Entity::find()
            .filter(
                planet_role::Column::Id.is_in(
                    positions
                        .iter()
                        .map(|p| p.role.clone())
                        .collect::<Vec<String>>(),
                ),
            )
            .all(db)
            .await
            .map_err(|_| errors::create_internal_server_error(None, "ROLES_RETRIEVAL_ERROR"))
    }
}
