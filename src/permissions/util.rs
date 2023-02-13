use super::checks;
use crate::entities::{planet, planet_member, planet_role};
use crate::errors;
use async_graphql::Error;
use log::error;
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};

/// Gets a planet. If an error occurs or the planet is not found, an error ready for presentation to
/// the client is returned.
pub async fn get_planet(id: String, db: &DatabaseConnection) -> Result<planet::Model, Error> {
    match planet::Entity::find_by_id(id.clone()).one(db).await {
        Ok(planet) => match planet {
            Some(planet) => Ok(planet),
            None => {
                error!(
                    "Planet {} no longer exists but some data with it was not scrubbed",
                    id
                );
                Err(errors::create_not_found_error())
            }
        },
        Err(_err) => Err(errors::create_internal_server_error(
            None,
            "PLANET_RETRIEVAL_ERROR",
        )),
    }
}

/// Gets a planet member. If an error occurs, an error ready for presentation to the client
/// is returned.
pub async fn get_planet_member(
    user_id: Option<String>,
    planet_id: String,
    db: &DatabaseConnection,
) -> Result<Option<planet_member::Model>, Error> {
    match user_id {
        Some(user_id) => match planet_member::Entity::find()
            .filter(
                planet_member::Column::User
                    .eq(user_id)
                    .and(planet_member::Column::Planet.eq(planet_id)),
            )
            .one(db)
            .await
        {
            Ok(member) => Ok(member),
            Err(_err) => Err(errors::create_internal_server_error(
                None,
                "MEMBER_RETRIEVAL_ERROR",
            )),
        },
        None => Ok(None),
    }
}

/// Gets the roles associated with a planet member. If an error occurs, an error ready for
/// presentation to the client is returned.
///
/// Note that the planet member input is an option: this function is designed to directly take the
/// input of get planet member.
pub async fn get_member_roles(
    planet_member: Option<planet_member::Model>,
    db: &DatabaseConnection,
) -> Result<Option<Vec<planet_role::Model>>, Error> {
    match planet_member {
        Some(member) => {
            match planet_role::Entity::find()
                .filter(planet_role::Column::Id.is_in(member.roles))
                .all(db)
                .await
            {
                Ok(roles) => Ok(Some(roles)),
                Err(_err) => Err(errors::create_internal_server_error(
                    None,
                    "ROLES_RETRIEVAL_ERROR",
                )),
            }
        }
        None => Ok(None),
    }
}

/// Checks for a permission and returns an error if that permission is not held by the user. This
/// function exists to ensure permission behavior is consistent across the API.
pub fn check_permission(
    permission: String,
    planet: planet::Model,
    member: Option<planet_member::Model>,
    roles: Option<Vec<planet_role::Model>>,
) -> Result<(), Error> {
    if checks::has_permission(permission, planet, member, roles) {
        Ok(())
    } else {
        Err(errors::create_not_found_error())
    }
}
