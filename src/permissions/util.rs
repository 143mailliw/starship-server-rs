use super::checks;
use crate::entities::{planet, planet_member, planet_role, user};
use crate::errors;
use crate::permissions::constants;
use async_graphql::Error;
use libreauth::oath::TOTPBuilder;
use sea_orm::{
    ActiveModelTrait, ActiveValue, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter,
};

/// Gets a planet. If an error occurs or the planet is not found, an error ready for presentation to
/// the client is returned.
pub async fn get_planet(id: String, db: &DatabaseConnection) -> Result<planet::Model, Error> {
    planet::Entity::find_by_id(id.clone())
        .one(db)
        .await
        .map_err(|_| errors::create_internal_server_error(None, "PLANET_RETRIEVAL_ERROR"))?
        .ok_or(errors::create_not_found_error())
}

/// Gets a planet member. If an error occurs, an error ready for presentation to the client
/// is returned.
pub async fn get_planet_member(
    user_id: Option<String>,
    planet_id: String,
    db: &DatabaseConnection,
) -> Result<Option<planet_member::Model>, Error> {
    match user_id {
        Some(user_id) => planet_member::Entity::find()
            .filter(
                planet_member::Column::User
                    .eq(user_id)
                    .and(planet_member::Column::Planet.eq(planet_id)),
            )
            .one(db)
            .await
            .map_err(|_| errors::create_internal_server_error(None, "MEMBER_RETRIEVAL_ERROR")),
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
        Some(member) => planet_role::Entity::find()
            .filter(planet_role::Column::Id.is_in(member.roles))
            .all(db)
            .await
            .map_err(|_| errors::create_internal_server_error(None, "ROLES_RETRIEVAL_ERROR"))
            .map(Some),
        None => Ok(None),
    }
}

/// Checks for a permission and returns an error if that permission is not held by the user. This
/// function exists to ensure permission behavior is consistent across the API.
pub fn check_permission(
    permission: &str,
    planet: &planet::Model,
    member: Option<planet_member::Model>,
    roles: Option<Vec<planet_role::Model>>,
) -> Result<(), Error> {
    if checks::has_permission(permission, planet, member, roles) {
        Ok(())
    } else {
        Err(errors::create_not_found_error())
    }
}

/// Checks to see whether or not the user's lowest positioned role is lower than the origin's lowest
/// position role, to be used in determining whether or not a role can be modified. This function
/// exists to ensure permission behavior is consistent across the API.
pub fn low_enough(
    user_roles: Option<Vec<planet_role::Model>>,
    origin_roles: Vec<planet_role::Model>,
    member: Option<planet_member::Model>,
) -> Result<(), Error> {
    if checks::low_enough(user_roles, origin_roles, member) {
        Ok(())
    } else {
        Err(errors::create_not_found_error())
    }
}

/// Modifies a permission vector using the prefixes from a vector of strings.
///
/// Prefixes:
/// '+' grants the permission.
/// '*' (or any other unspecified character) falls back to the previous permission set.
/// '-' explicitly denies the permission.
pub fn update_permissions(
    mut destination_vec: Vec<String>,
    permissions: Vec<String>,
) -> Result<Vec<String>, Error> {
    for permission in permissions {
        // remove the permission from the array
        let mut permission_chars = permission.chars();
        let permission_prefix = permission_chars.next();
        let base_permission: String = permission_chars.collect();

        destination_vec.retain(|p| {
            let mut p_chars = p.chars();
            p_chars.next();
            let base_p: String = p_chars.collect();

            base_p != base_permission
        });

        if constants::MEMBER_PERMISSIONS
            .iter()
            .any(|&s| s == permission)
        {
            return Err(errors::create_user_input_error(
                &format!("You cannot change the '{permission}' permission."),
                "SPECIAL_PERMISSION",
            ));
        }

        if permission_prefix == Some('-') || permission_prefix == Some('+') {
            destination_vec.push(permission);
        }
    }

    Ok(destination_vec)
}

/// Verifies a two factor authentication token. If a token is required and the token provided is
/// invalid, an error will be returned.
pub async fn verify_token(
    db: &DatabaseConnection,
    user: &user::Model,
    token: Option<u32>,
) -> Result<bool, Error> {
    if user.tfa_enabled {
        if let Some(token) = token {
            let is_valid = TOTPBuilder::new()
                .hex_key(user.tfa_secret.as_ref().unwrap())
                .finalize()
                .map_err(|_| errors::create_internal_server_error(None, "TOTP_BUILD_ERROR"))?
                .is_valid(&token.to_string());

            if is_valid || user.tfa_backup.contains(&token.to_string()) {
                if user.tfa_backup.contains(&token.to_string()) {
                    let mut remaining_codes = user.tfa_backup.clone();
                    remaining_codes.retain(|searched_code| searched_code != &token.to_string());

                    let mut active_user: user::ActiveModel = user.clone().into();
                    active_user.tfa_backup = ActiveValue::Set(remaining_codes);

                    active_user.update(db).await.map_err(|_| {
                        errors::create_internal_server_error(None, "UPDATE_USER_ERROR")
                    })?;
                }

                Ok(true)
            } else {
                Err(errors::create_user_input_error(
                    "Incorrect TFA token or backup code.",
                    "INCORRECT_CODE",
                ))
            }
        } else {
            Err(errors::create_user_input_error(
                "No TFA token or backup code was provided.",
                "NO_CODE",
            ))
        }
    } else {
        Ok(true)
    }
}
