use crate::components::index::{create_component, delete_component};
use crate::entities::planet_component;
use crate::errors;
use crate::permissions::util;
use crate::sessions::Session;
use async_graphql::{Context, Description, Error, Object, ID};
use nanoid::nanoid;
use sea_orm::{ActiveModelTrait, ActiveValue, DatabaseConnection, EntityTrait, QueryOrder};

#[derive(Default, Description)]
pub struct ComponentMutation;

#[Object(rename_fields = "camelCase", rename_args = "camelCase")]
impl ComponentMutation {
    /// Creates a component.
    #[graphql(complexity = 50)]
    async fn create_component(
        &self,
        ctx: &Context<'_>,
        planet_id: ID,
        name: String,
        component: String,
    ) -> Result<planet_component::Model, Error> {
        let db = ctx.data::<DatabaseConnection>().unwrap();
        let session = ctx.data::<Session>().unwrap();
        let user_id = session.user.as_ref().map(|user| user.id.clone());

        let planet = util::get_planet(planet_id.to_string(), db).await?;
        let member = util::get_planet_member(user_id.clone(), planet_id.to_string(), db).await?;
        let roles = util::get_member_roles(member.clone(), db).await?;
        util::check_permission("planet.component.create", &planet, member, roles)?;

        let component_id =
            create_component(component.as_str(), planet_id.to_string(), user_id.unwrap()).await?;

        let position = planet_component::Entity::find()
            .order_by_asc(planet_component::Column::Position)
            .one(db)
            .await
            .map_err(|_| errors::create_internal_server_error(None, "ORDER_RETRIEVAL_ERROR"))?
            .map_or(0, |component| component.position)
            - 1;

        let component = planet_component::ActiveModel {
            id: ActiveValue::Set(nanoid!(16)),
            r#type: ActiveValue::Set(component),
            component_id: ActiveValue::Set(component_id),
            name: ActiveValue::Set(name),
            planet: ActiveValue::Set(planet_id.to_string()),
            created: ActiveValue::Set(chrono::offset::Utc::now().naive_utc()),
            position: ActiveValue::Set(position),
            ..Default::default()
        };

        let component_result = planet_component::Entity::insert(component)
            .exec(db)
            .await
            .map_err(|_| errors::create_internal_server_error(None, "INSERTION_ERROR"))?;

        planet_component::Entity::find_by_id(component_result.last_insert_id)
            .one(db)
            .await
            .map_err(|_| errors::create_internal_server_error(None, "RETRIEVAL_ERROR"))?
            .ok_or(errors::create_internal_server_error(None, "MISSING_ERROR"))
    }

    /// Deletes a component.
    #[graphql(complexity = 200)]
    async fn delete_component(
        &self,
        ctx: &Context<'_>,
        id: ID,
        token: Option<u32>,
    ) -> Result<bool, Error> {
        let db = ctx.data::<DatabaseConnection>().unwrap();
        let session = ctx.data::<Session>().unwrap();
        let user_id = session.user.as_ref().map(|user| user.id.clone());

        let component = planet_component::Entity::find_by_id(id.to_string())
            .one(db)
            .await
            .map_err(|_| errors::create_internal_server_error(None, "RETRIEVAL_ERROR"))?
            .ok_or(errors::create_not_found_error())?;

        let planet = util::get_planet(component.clone().planet, db).await?;
        let member = util::get_planet_member(user_id, component.clone().planet, db).await?;
        let roles = util::get_member_roles(member.clone(), db).await?;
        util::check_permission("planet.component.delete", &planet, member, roles)?;

        util::verify_token(db, session.user.as_ref().unwrap(), token).await?;
        if planet.home == Some(component.id.clone()) {
            return Err(errors::create_user_input_error(
                "You can't delete the home component.",
                "HOME",
            ));
        }

        delete_component(&component.r#type, component.id.clone()).await?;

        let active_component: planet_component::ActiveModel = component.into();

        active_component
            .delete(db)
            .await
            .map_err(|_| errors::create_internal_server_error(None, "DELETE_PLANET_ERROR"))
            .map(|_| true)
    }

    /// Renames a component.
    #[graphql(complexity = 10)]
    async fn rename_component(
        &self,
        ctx: &Context<'_>,
        id: ID,
        name: String,
    ) -> Result<planet_component::Model, Error> {
        let db = ctx.data::<DatabaseConnection>().unwrap();
        let session = ctx.data::<Session>().unwrap();
        let user_id = session.user.as_ref().map(|user| user.id.clone());

        let component = planet_component::Entity::find_by_id(id.to_string())
            .one(db)
            .await
            .map_err(|_| errors::create_internal_server_error(None, "RETRIEVAL_ERROR"))?
            .ok_or(errors::create_not_found_error())?;

        let planet = util::get_planet(component.clone().planet, db).await?;
        let member = util::get_planet_member(user_id, component.clone().planet, db).await?;
        let roles = util::get_member_roles(member.clone(), db).await?;
        util::check_permission("planet.component.rename", &planet, member, roles)?;

        let mut active_component: planet_component::ActiveModel = component.into();
        active_component.name = ActiveValue::Set(name);

        active_component
            .update(db)
            .await
            .map_err(|_| errors::create_internal_server_error(None, "UPDATE_ERROR"))
    }
}
