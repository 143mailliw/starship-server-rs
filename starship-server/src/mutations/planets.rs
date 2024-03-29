use crate::entities::{planet, planet_component, planet_member, planet_role};
use crate::errors;
use crate::guards::session::{SessionGuard, SessionType};
use crate::permissions::{constants, util};
use crate::sessions::Session;
use async_graphql::{Context, Description, Error, Object, ID};
use nanoid::nanoid;
use sea_orm::{
    ActiveModelTrait, ActiveValue, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter,
};

#[derive(Default, Description)]
pub struct PlanetMutation;

#[Object(rename_fields = "camelCase", rename_args = "camelCase")]
impl PlanetMutation {
    /// Creates a new planet.
    #[graphql(guard = "SessionGuard::new(SessionType::User)", complexity = 200)]
    async fn insert_planet(
        &self,
        ctx: &Context<'_>,
        name: String,
        private: bool,
    ) -> Result<planet::Model, Error> {
        let db = ctx.data::<DatabaseConnection>().unwrap();
        let session = ctx.data::<Session>().unwrap();
        // unwrap is safe because guard guarantees we have a user
        let user = session.user.as_ref().unwrap();

        // create the planet
        if name.len() > 128 {
            return Err(errors::create_user_input_error(
                "Planet name cannot be longer than 128 characters.",
                "NAME_TOO_LONG",
            ));
        }

        let mut planet = planet::ActiveModel {
            id: ActiveValue::Set(nanoid!(16)),
            name: ActiveValue::Set(name),
            created: ActiveValue::Set(chrono::offset::Utc::now().naive_utc()),
            owner: ActiveValue::Set(user.id.clone()),
            private: ActiveValue::Set(private),
            member_count: ActiveValue::Set(1),
            ..Default::default()
        };

        let result = planet::Entity::insert(planet.clone())
            .exec(db)
            .await
            .map_err(|_| errors::create_internal_server_error(None, "PLANET_INSERTION_ERROR"))?;

        let role = planet_role::ActiveModel {
            id: ActiveValue::Set(nanoid!(16)),
            name: ActiveValue::Set("Default".to_string()),
            color: ActiveValue::Set("#FFFFFF".to_string()),
            permissions: ActiveValue::Set(
                constants::VIEWER_PERMISSIONS
                    .iter()
                    .chain(constants::MEMBER_PERMISSIONS.iter())
                    .map(std::string::ToString::to_string)
                    .collect(),
            ),
            planet: ActiveValue::Set(result.last_insert_id.clone()),
            position: ActiveValue::Set(0),
            default: ActiveValue::Set(true),
        };

        let role_result = planet_role::Entity::insert(role)
            .exec(db)
            .await
            .map_err(|_| errors::create_internal_server_error(None, "ROLE_INSERTION_ERROR"))?;

        let member = planet_member::ActiveModel {
            id: ActiveValue::Set(nanoid!(16)),
            planet: ActiveValue::Set(result.last_insert_id.clone()),
            user: ActiveValue::Set(user.id.clone()),
            roles: ActiveValue::Set(vec![role_result.last_insert_id]),
            permissions: ActiveValue::Set(vec!["+owner".to_string()]),
            created: ActiveValue::Set(chrono::offset::Utc::now().naive_utc()),
            banned: ActiveValue::Set(false),
        };

        planet_member::Entity::insert(member)
            .exec(db)
            .await
            .map_err(|_| errors::create_internal_server_error(None, "MEMBER_INSERTION_ERROR"))?;

        // TODO: make this create a page instead
        let component = planet_component::ActiveModel {
            id: ActiveValue::Set(nanoid!(16)),
            r#type: ActiveValue::Set("dummy".to_string()),
            component_id: ActiveValue::Set("dummy".to_string()),
            name: ActiveValue::Set("Dummy Component".to_string()),
            planet: ActiveValue::Set(result.last_insert_id),
            created: ActiveValue::Set(chrono::offset::Utc::now().naive_utc()),
            ..Default::default() // i don't know what's up with this but we have to have it
        };

        let component_result = planet_component::Entity::insert(component)
            .exec(db)
            .await
            .map_err(|_| errors::create_internal_server_error(None, "COMPONENT_INSERTION_ERROR"))?;

        planet.home = ActiveValue::Set(Some(component_result.last_insert_id));

        planet
            .update(db)
            .await
            .map_err(|_| errors::create_internal_server_error(None, "UPDATE_ERROR"))
    }

    /// Renames a planet.
    #[graphql(complexity = 10)]
    async fn rename_planet(
        &self,
        ctx: &Context<'_>,
        id: ID,
        name: String,
    ) -> Result<planet::Model, Error> {
        let db = ctx.data::<DatabaseConnection>().unwrap();
        let session = ctx.data::<Session>().unwrap();
        let user_id = session.user.as_ref().map(|user| user.id.clone());

        let planet = util::get_planet(id.to_string(), db).await?;
        let member = util::get_planet_member(user_id, id.to_string(), db).await?;
        let roles = util::get_member_roles(member.clone(), db).await?;
        util::check_permission("planet.change_name", &planet, member, roles)?;

        let mut active_planet: planet::ActiveModel = planet.into();
        active_planet.name = ActiveValue::Set(name);

        active_planet
            .update(db)
            .await
            .map_err(|_| errors::create_internal_server_error(None, "UPDATE_ERROR"))
    }

    /// Sets a planet's description.
    #[graphql(complexity = 10)]
    async fn set_planet_description(
        &self,
        ctx: &Context<'_>,
        id: ID,
        description: String,
    ) -> Result<planet::Model, Error> {
        let db = ctx.data::<DatabaseConnection>().unwrap();
        let session = ctx.data::<Session>().unwrap();
        let user_id = session.user.as_ref().map(|user| user.id.clone());

        let planet = util::get_planet(id.to_string(), db).await?;
        let member = util::get_planet_member(user_id, id.to_string(), db).await?;
        let roles = util::get_member_roles(member.clone(), db).await?;
        util::check_permission("planet.change_name", &planet, member, roles)?;

        let mut active_planet: planet::ActiveModel = planet.into();
        active_planet.description = ActiveValue::Set(Some(description));

        active_planet
            .update(db)
            .await
            .map_err(|_| errors::create_internal_server_error(None, "UPDATE_ERROR"))
    }

    /// Toggles whether or not a planet is private.
    #[graphql(complexity = 50)]
    async fn toggle_private(
        &self,
        ctx: &Context<'_>,
        id: ID,
        token: Option<u32>,
    ) -> Result<planet::Model, Error> {
        let db = ctx.data::<DatabaseConnection>().unwrap();
        let session = ctx.data::<Session>().unwrap();
        let user_id = session.user.as_ref().map(|user| user.id.clone());

        let planet = util::get_planet(id.to_string(), db).await?;
        let member = util::get_planet_member(user_id, id.to_string(), db).await?;
        let roles = util::get_member_roles(member.clone(), db).await?;
        util::check_permission("planet.change_publicity", &planet, member, roles)?;

        util::verify_token(db, session.user.as_ref().unwrap(), token).await?;

        let mut active_planet: planet::ActiveModel = planet.into();
        active_planet.private = ActiveValue::Set(!active_planet.private.unwrap());

        active_planet
            .update(db)
            .await
            .map_err(|_| errors::create_internal_server_error(None, "UPDATE_ERROR"))
    }

    /// Deletes a planet and all of it's associated data immediately. This action is unrecoverable.
    #[graphql(complexity = 200)]
    async fn delete_planet(
        &self,
        ctx: &Context<'_>,
        id: ID,
        token: Option<u32>,
    ) -> Result<bool, Error> {
        let db = ctx.data::<DatabaseConnection>().unwrap();
        let session = ctx.data::<Session>().unwrap();
        let user_id = session.user.as_ref().map(|user| user.id.clone());

        let planet = util::get_planet(id.to_string(), db).await?;
        let member = util::get_planet_member(user_id, id.to_string(), db).await?;
        let roles = util::get_member_roles(member.clone(), db).await?;
        util::check_permission("planet.change_publicity", &planet, member, roles)?;

        util::verify_token(db, session.user.as_ref().unwrap(), token).await?;

        // clear out circular references
        let mut active_planet: planet::ActiveModel = planet.clone().into();

        active_planet.home = ActiveValue::Set(None);

        active_planet
            .clone()
            .update(db)
            .await
            .map_err(|_| errors::create_internal_server_error(None, "UPDATE_ERROR"))?;

        // TODO: remove all component data

        planet_member::Entity::delete_many()
            .filter(planet_member::Column::Planet.eq(planet.id.clone()))
            .exec(db)
            .await
            .map_err(|_| errors::create_internal_server_error(None, "DELETE_MEMBERS_ERROR"))?;

        planet_role::Entity::delete_many()
            .filter(planet_role::Column::Planet.eq(planet.id.clone()))
            .exec(db)
            .await
            .map_err(|_| errors::create_internal_server_error(None, "DELETE_ROLES_ERROR"))?;

        planet_component::Entity::delete_many()
            .filter(planet_component::Column::Planet.eq(planet.id.clone()))
            .exec(db)
            .await
            .map_err(|_| errors::create_internal_server_error(None, "DELETE_COMPONENTS_ERROR"))?;

        active_planet
            .delete(db)
            .await
            .map_err(|_| errors::create_internal_server_error(None, "DELETE_PLANET_ERROR"))
            .map(|_| true)
    }

    /// Changes the home component of a planet.
    #[graphql(complexity = 50)]
    async fn set_home_component(&self, ctx: &Context<'_>, id: ID) -> Result<planet::Model, Error> {
        let db = ctx.data::<DatabaseConnection>().unwrap();
        let session = ctx.data::<Session>().unwrap();
        let user_id = session.user.as_ref().map(|user| user.id.clone());

        let component = planet_component::Entity::find_by_id(id.to_string())
            .one(db)
            .await
            .map_err(|_| errors::create_internal_server_error(None, "RETRIEVAL_ERROR"))?
            .ok_or(errors::create_not_found_error())?;

        let planet = util::get_planet(component.planet.clone(), db).await?;
        let member = util::get_planet_member(user_id.clone(), component.planet.clone(), db).await?;
        let roles = util::get_member_roles(member.clone(), db).await?;
        util::check_permission("planet.component.set_home", &planet, member, roles)?;

        if component.parent_id.is_some() {
            return Err(errors::create_user_input_error(
                "Home planet cannot have a parent.",
                "HAS_PARENT",
            ));
        }

        let mut active_planet: planet::ActiveModel = planet.into();
        active_planet.home = ActiveValue::Set(Some(component.id));

        active_planet
            .update(db)
            .await
            .map_err(|_| errors::create_internal_server_error(None, "UPDATE_ERROR"))
    }
}
