use crate::entities::{planet, planet_member, planet_role};
use crate::errors;
use crate::guards::session::{SessionGuard, SessionType};
use crate::permissions::{constants, util};
use crate::sessions::Session;
use async_graphql::{Context, Description, Error, Object, ID};
use log::error;
use nanoid::nanoid;
use sea_orm::{
    ActiveModelTrait, ActiveValue, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter,
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
            .map_err(|_| errors::create_internal_server_error(None, "PLANET_RETRIEVAL_ERROR"))?;

        if let Some(planet) = planet {
            let role = planet_role::Entity::find()
                .filter(
                    planet_role::Column::Planet
                        .eq(planet.id.clone())
                        .and(planet_role::Column::Default.eq(true)),
                )
                .one(db)
                .await
                .map_err(|_| errors::create_internal_server_error(None, "ROLE_RETRIEVAL_ERROR"))?;

            if let Some(role) = role {
                if !planet.private {
                    if planet_member::Entity::find()
                        .filter(planet_member::Column::User.eq(user_id.as_ref().unwrap().clone()))
                        .one(db)
                        .await
                        .map_err(|_| {
                            errors::create_internal_server_error(
                                None,
                                "MEMBER_CHECK_RETRIEVAL_ERROR",
                            )
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
                    };

                    match planet_member::Entity::insert(member).exec(db).await {
                        Ok(value) => {
                            match planet_member::Entity::find_by_id(value.last_insert_id)
                                .one(db)
                                .await
                            {
                                Ok(value) => match value {
                                    Some(member) => Ok(member),
                                    None => Err(errors::create_internal_server_error(
                                        None,
                                        "FIND_ERROR",
                                    )),
                                },
                                Err(_err) => Err(errors::create_internal_server_error(
                                    None,
                                    "MEMBER_RETRIEVAL_ERROR",
                                )),
                            }
                        }
                        Err(error) => {
                            error!("{}", error);
                            Err(errors::create_internal_server_error(
                                None,
                                "INSERTION_ERROR",
                            ))
                        }
                    }
                } else {
                    Err(errors::create_not_found_error())
                }
            } else {
                Err(errors::create_internal_server_error(
                    None,
                    "MISSING_DEFAULT_ROLE_ERROR",
                ))
            }
        } else {
            Err(errors::create_not_found_error())
        }
    }
}
