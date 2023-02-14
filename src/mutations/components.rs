#![allow(non_snake_case)]
use crate::entities::planet_component;
use crate::errors;
use crate::permissions::util;
use crate::sessions::Session;
use async_graphql::{Context, Description, Error, Object, ID};
use log::error;
use nanoid::nanoid;
use sea_orm::{ActiveModelTrait, ActiveValue, DatabaseConnection, EntityTrait};

#[derive(Default, Description)]
pub struct ComponentMutation;

#[Object]
impl ComponentMutation {
    async fn renameComponent(
        &self,
        ctx: &Context<'_>,
        id: ID,
        name: String,
    ) -> Result<planet_component::Model, Error> {
        let db = ctx.data::<DatabaseConnection>().unwrap();
        let session = ctx.data::<Session>().unwrap();
        let user_id = session.user.as_ref().map(|user| user.id.clone());

        match planet_component::Entity::find_by_id(id.to_string())
            .one(db)
            .await
        {
            Ok(value) => match value {
                Some(component) => {
                    let planet = util::get_planet(component.clone().planet, db).await?;
                    let member =
                        util::get_planet_member(user_id, component.clone().planet, db).await?;
                    let roles = util::get_member_roles(member.clone(), db).await?;
                    util::check_permission(
                        "planet.component.rename".to_string(),
                        planet.clone(),
                        member,
                        roles,
                    )?;

                    let mut active_component: planet_component::ActiveModel = component.into();
                    active_component.name = ActiveValue::Set(name);

                    match active_component.update(db).await {
                        Ok(value) => Ok(value),
                        Err(_err) => {
                            Err(errors::create_internal_server_error(None, "UPDATE_ERROR"))
                        }
                    }
                }
                None => Err(errors::create_not_found_error()),
            },
            Err(_err) => Err(errors::create_internal_server_error(
                None,
                "RETRIEVAL_ERROR",
            )),
        }
    }
}
