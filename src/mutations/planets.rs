#![allow(non_snake_case)]
use crate::entities::{planet, planet_component, planet_member, planet_role};
use crate::errors;
use crate::guards::session::{SessionGuard, SessionType};
use crate::permissions::constants;
use crate::sessions::Session;
use async_graphql::{Context, Description, Error, Object, SimpleObject, ID};
use log::error;
use nanoid::nanoid;
use sea_orm::{ActiveModelTrait, ActiveValue, DatabaseConnection, EntityTrait, TryIntoModel};

#[derive(Default, Description)]
pub struct PlanetMutation;

#[Object]
impl PlanetMutation {
    /// Creates a new planet.
    #[graphql(guard = "SessionGuard::new(SessionType::User)", complexity = 200)]
    async fn insertPlanet(
        &self,
        ctx: &Context<'_>,
        name: String,
        private: bool,
    ) -> Result<planet::Model, Error> {
        // Result<planet::Model, Error> {
        let db = ctx.data::<DatabaseConnection>().unwrap();
        let session = ctx.data::<Session>().unwrap();
        // unwrap is safe because guard guarantees we have a user
        let user = session.user.clone().unwrap();

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
            banned: ActiveValue::Set(vec![]),
            ..Default::default()
        };

        let result = match planet::Entity::insert(planet.clone()).exec(db).await {
            Ok(value) => value,
            Err(error) => {
                error!("{}", error);
                return Err(errors::create_internal_server_error(
                    None,
                    "PLANET_INSERTION_ERROR",
                ));
            }
        };

        // retrieve the planet again to make sure all the info we have is synced with the db
        // let final_planet = match planet::Entity::find_by_id(result.last_insert_id)
        //     .one(db)
        //     .await
        // {
        //     Ok(value) => match value {
        //         Some(value) => Ok(value),
        //         None => Err(errors::create_internal_server_error(
        //             None,
        //             "PLANET_BAD_ID_ERROR",
        //         )),
        //     },
        //     Err(error) => {
        //         error!("{}", error);
        //         Err(errors::create_internal_server_error(
        //             None,
        //             "PLANET_RETRIEVAL_ERROR",
        //         ))
        //     }
        // };

        let role = planet_role::ActiveModel {
            id: ActiveValue::Set(nanoid!(16)),
            name: ActiveValue::Set("Default".to_string()),
            color: ActiveValue::Set("#FFFFFF".to_string()),
            permissions: ActiveValue::Set(
                constants::DEFAULT_PERMISSIONS
                    .iter()
                    .map(|perm| perm.to_string())
                    .collect(),
            ),
            planet: ActiveValue::Set(result.last_insert_id.clone()),
            position: ActiveValue::Set(0),
            default: ActiveValue::Set(true),
        };

        let role_result = match planet_role::Entity::insert(role).exec(db).await {
            Ok(value) => value,
            Err(error) => {
                error!("{}", error);
                return Err(errors::create_internal_server_error(
                    None,
                    "ROLE_INSERTION_ERROR",
                ));
            }
        };

        let member = planet_member::ActiveModel {
            id: ActiveValue::Set(nanoid!(16)),
            planet: ActiveValue::Set(result.last_insert_id.clone()),
            user: ActiveValue::Set(user.id),
            roles: ActiveValue::Set(vec![role_result.last_insert_id]),
            permissions: ActiveValue::Set(vec!["owner".to_string()]),
            created: ActiveValue::Set(chrono::offset::Utc::now().naive_utc()),
        };

        match planet_member::Entity::insert(member).exec(db).await {
            Ok(_value) => (),
            Err(error) => {
                error!("{}", error);
                return Err(errors::create_internal_server_error(
                    None,
                    "MEMBER_INSERTION_ERROR",
                ));
            }
        }

        // TODO: make this create a page instead
        let component = planet_component::ActiveModel {
            id: ActiveValue::Set(nanoid!(16)),
            r#type: ActiveValue::Set("dummy".to_string()),
            component_id: ActiveValue::Set("dummy".to_string()),
            name: ActiveValue::Set("Dummy Component".to_string()),
            planet: ActiveValue::Set(result.last_insert_id.clone()),
            created: ActiveValue::Set(chrono::offset::Utc::now().naive_utc()),
            ..Default::default() // i don't know what's up with this but we have to have it
        };

        let component_result = match planet_component::Entity::insert(component).exec(db).await {
            Ok(value) => value,
            Err(error) => {
                error!("{}", error);
                return Err(errors::create_internal_server_error(
                    None,
                    "COMPONENT_INSERTION_ERROR",
                ));
            }
        };

        planet.home = ActiveValue::Set(Some(component_result.last_insert_id));

        match planet.update(db).await {
            Ok(value) => Ok(value),
            Err(error) => {
                error!("{}", error);
                Err(errors::create_internal_server_error(None, "UPDATE_ERROR"))
            }
        }
    }
}
