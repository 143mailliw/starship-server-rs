#![allow(non_snake_case)]
use crate::entities::prelude::Token;
use crate::entities::prelude::User;
use crate::entities::token;
use crate::entities::user;
use crate::errors;
use crate::guards::session::{SessionGuard, SessionType};
use crate::sessions::{JWTLoginToken, Session};
use async_graphql::{Context, Description, Error, Object, SimpleObject, ID};
use bcrypt::hash;
use email_address::EmailAddress;
use hmac::{Hmac, Mac};
use jwt::SignWithKey;
use libreauth::key::KeyBuilder;
use libreauth::oath::{TOTPBuilder, TOTP};
use log::error;
use nanoid::nanoid;
use sea_orm::{
    ActiveModelTrait, ActiveValue, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter,
};
use sha2::Sha256;
use std::env;

#[derive(SimpleObject)]
struct LoginPayload {
    token: String,
    expectingTFA: bool,
}

#[derive(Default, Description)]
pub struct UserMutation;

#[Object]
impl UserMutation {
    /// Registers a new user.
    #[graphql(complexity = 200)]
    async fn insertUser(
        &self,
        ctx: &Context<'_>,
        email: String,
        password: String,
        username: String,
        recaptcha: String,
    ) -> Result<user::Model, Error> {
        // TODO: add RECAPTCHA support
        // TODO: verify email

        let db = ctx.data::<DatabaseConnection>().unwrap();

        match User::find()
            .filter(
                user::Column::Username
                    .eq(username.clone())
                    .or(user::Column::EmailAddress.eq(email.clone())),
            )
            .one(db)
            .await
        {
            Ok(value) => {
                if let Some(value) = value {
                    if value.email_address == email {
                        return Err(errors::create_user_input_error(
                            "An account is already registered with that email.",
                            "EMAIL_ALREADY_EXISTS",
                        ));
                    }

                    if value.username == username {
                        return Err(errors::create_user_input_error(
                            "An account is already registered with that username",
                            "USERNAME_ALREADY_EXISTS",
                        ));
                    }
                }
            }
            Err(error) => {
                error!("{}", error);
                return Err(errors::create_internal_server_error(None, "FIND_ERROR"));
            }
        };

        if username.len() < 4 {
            return Err(errors::create_user_input_error(
                "Your username must be at least 4 characters.",
                "USERNAME_TOO_SHORT",
            ));
        };

        if !EmailAddress::is_valid(&email.to_owned()) {
            return Err(errors::create_user_input_error(
                "Invalid email address.",
                "INVALID_EMAIL_ADDRESS",
            ));
        };

        // inputs are valid
        let hash = match hash(password, 4) {
            Ok(value) => value,
            Err(error) => {
                error!("{}", error);
                return Err(errors::create_internal_server_error(None, "HASH_ERROR"));
            }
        };

        let user = user::ActiveModel {
            id: ActiveValue::Set(nanoid!(16)),
            username: ActiveValue::Set(username),
            password: ActiveValue::Set(hash),
            email_address: ActiveValue::Set(email),
            created: ActiveValue::Set(chrono::offset::Utc::now().naive_utc()),
            blocked: ActiveValue::Set(vec![]),
            sessions: ActiveValue::Set(vec![]),
            tfa_backup: ActiveValue::set(vec![]),
            ..Default::default()
        };

        let result = match User::insert(user).exec(db).await {
            Ok(value) => value,
            Err(error) => {
                error!("{}", error);
                return Err(errors::create_internal_server_error(
                    None,
                    "INSERTION_ERROR",
                ));
            }
        };

        match User::find_by_id(result.last_insert_id).one(db).await {
            Ok(value) => match value {
                Some(value) => Ok(value),
                None => Err(errors::create_internal_server_error(None, "BAD_ID_ERROR")),
            },
            Err(error) => {
                error!("{}", error);
                Err(errors::create_internal_server_error(
                    None,
                    "RETRIEVAL_ERROR",
                ))
            }
        }
    }

    /// Creates a new token & and signs a JWT object containing it's ID.
    #[graphql(complexity = 200)]
    async fn loginUser(
        &self,
        ctx: &Context<'_>,
        username: String,
        password: String,
    ) -> Result<LoginPayload, Error> {
        let db = ctx.data::<DatabaseConnection>().unwrap();
        let session = ctx.data::<Session>().unwrap();

        let user = match User::find()
            .filter(user::Column::Username.eq(username.clone()))
            .one(db)
            .await
        {
            Ok(value) => match value {
                Some(value) => value,
                None => {
                    return Err(errors::create_forbidden_error(
                        Some("Invalid username or password."),
                        "INVALID_USER",
                    ))
                }
            },
            Err(error) => {
                error!("{}", error);
                return Err(errors::create_internal_server_error(None, "FIND_ERROR"));
            }
        };

        if env::var("SMTP_HOST").is_ok() && !user.verified {
            return Err(errors::create_forbidden_error(
                Some("You need to verify your email."),
                "UNVERIFIED_EMAIL",
            ));
        };

        match bcrypt::verify(password, &user.password) {
            Ok(result) => {
                if !result {
                    return Err(errors::create_forbidden_error(
                        Some("Invalid username or password."),
                        "INVALID_USER",
                    ));
                }
            }
            Err(error) => {
                error!("{}", error);
                return Err(errors::create_internal_server_error(
                    None,
                    "VERIFICATION_ERROR",
                ));
            }
        };

        //TODO: Find approx. location from IP
        //TODO: Find OS and Browser

        let addr = match session.ip_address {
            Some(value) => value.to_string(),
            None => "0.0.0.0".to_string(),
        };

        let token = token::ActiveModel {
            id: ActiveValue::Set(nanoid!(16)),
            user: ActiveValue::Set(user.id.clone()),
            ip: ActiveValue::Set(addr),
            location: ActiveValue::Set("Unknown".to_string()),
            latitude: ActiveValue::Set(None),
            longitude: ActiveValue::Set(None),
            browser: ActiveValue::Set("Unknown".to_string()),
            operating_system: ActiveValue::Set("Unknown".to_string()),
        };

        match Token::insert(token).exec(db).await {
            Ok(res) => {
                let jwt_data = JWTLoginToken {
                    token: res.last_insert_id,
                    user_id: user.id,
                };

                let secret = env::var("SECRET").unwrap();
                let key: Hmac<Sha256> = Hmac::new_from_slice(secret.as_bytes()).unwrap();
                let token = jwt_data.sign_with_key(&key).unwrap();

                Ok(LoginPayload {
                    token,
                    expectingTFA: user.tfa_enabled,
                })
            }
            Err(error) => {
                error!("{}", error);
                Err(errors::create_internal_server_error(
                    None,
                    "INSERTION_ERROR",
                ))
            }
        }
    }

    /// Toggles whether or not a user is banned.
    #[graphql(guard = "SessionGuard::new(SessionType::Admin)", complexity = 10)]
    async fn banUser(&self, ctx: &Context<'_>, userId: ID) -> Result<user::Model, Error> {
        let db = ctx.data::<DatabaseConnection>().unwrap();
        let id = userId.to_string();

        match User::find_by_id(id).one(db).await {
            Ok(value) => match value {
                Some(user) => {
                    if (user.admin) {
                        return Err(errors::create_forbidden_error(
                            Some("You cannot ban an administrator."),
                            "ADMINISTRATIVE_IMMUNITY",
                        ));
                    }

                    let mut active_user: user::ActiveModel = user.clone().into();
                    active_user.banned = ActiveValue::Set(!user.banned);

                    match active_user.update(db).await {
                        Ok(value) => Ok(value),
                        Err(error) => {
                            error!("{}", error);
                            Err(errors::create_internal_server_error(None, "UPDATE_ERROR"))
                        }
                    }
                }
                None => Err(errors::create_not_found_error()),
            },
            Err(error) => {
                error!("{}", error);
                Err(errors::create_internal_server_error(
                    None,
                    "RETRIEVAL_ERROR",
                ))
            }
        }
    }

    /// Toggles whether or not a user is blocked.
    #[graphql(guard = "SessionGuard::new(SessionType::User)", complexity = 10)]
    async fn toggleBlockUser(&self, ctx: &Context<'_>, userId: ID) -> Result<user::Model, Error> {
        let db = ctx.data::<DatabaseConnection>().unwrap();
        let session = ctx.data::<Session>().unwrap();
        let id = userId.to_string();

        match User::find_by_id(id).one(db).await {
            Ok(value) => match value {
                Some(user) => {
                    // unwrap is safe because guard guarantees we have a user
                    if user.id == session.user.as_ref().unwrap().id {
                        return Err(errors::create_user_input_error(
                            "You cannot block yourself.",
                            "INVALID_USER_SELF",
                        ));
                    }

                    let mut blocked = session.user.as_ref().unwrap().blocked.clone();
                    if blocked.contains(&user.id) {
                        blocked.retain(|searched_user| **searched_user != user.id)
                    } else {
                        blocked.push(user.id);
                    };

                    let mut active_user: user::ActiveModel = session.user.clone().unwrap().into();
                    active_user.blocked = ActiveValue::Set(blocked);

                    match active_user.update(db).await {
                        Ok(value) => Ok(value),
                        Err(error) => {
                            error!("{}", error);
                            Err(errors::create_internal_server_error(None, "UPDATE_ERROR"))
                        }
                    }
                }
                None => Err(errors::create_not_found_error()),
            },
            Err(error) => {
                error!("{}", error);
                Err(errors::create_internal_server_error(
                    None,
                    "RETRIEVAL_ERROR",
                ))
            }
        }
    }

    /// Changes the current user's profile bio.
    #[graphql(guard = "SessionGuard::new(SessionType::NotBanned)", complexity = 10)]
    async fn updateProfileBio(&self, ctx: &Context<'_>, bio: String) -> Result<user::Model, Error> {
        if bio.len() > 4000 {
            return Err(errors::create_user_input_error(
                "Your bio cannot be longer than 4000 characters.",
                "BIO_TOO_LONG",
            ));
        };

        let db = ctx.data::<DatabaseConnection>().unwrap();
        let session = ctx.data::<Session>().unwrap();

        let mut active_user: user::ActiveModel = session.user.clone().unwrap().into();
        active_user.profile_bio = ActiveValue::Set(Some(bio));

        match active_user.update(db).await {
            Ok(value) => Ok(value),
            Err(error) => {
                error!("{}", error);
                Err(errors::create_internal_server_error(None, "UPDATE_ERROR"))
            }
        }
    }

    /// Changes the current user's notification setting.
    #[graphql(guard = "SessionGuard::new(SessionType::User)", complexity = 10)]
    async fn setNotificationSetting(
        &self,
        ctx: &Context<'_>,
        option: i16,
    ) -> Result<user::Model, Error> {
        if !(0..=4).contains(&option) {
            return Err(errors::create_user_input_error(
                "Invalid notification setting.",
                "INVALID_SETTING",
            ));
        }
        let db = ctx.data::<DatabaseConnection>().unwrap();
        let session = ctx.data::<Session>().unwrap();

        let mut active_user: user::ActiveModel = session.user.clone().unwrap().into();
        active_user.notification_setting = ActiveValue::Set(option);

        match active_user.update(db).await {
            Ok(value) => Ok(value),
            Err(error) => {
                error!("{}", error);
                Err(errors::create_internal_server_error(None, "UPDATE_ERROR"))
            }
        }
    }

    #[graphql(guard = "SessionGuard::new(SessionType::User)", complexity = 10)]
    async fn generateTOTPSecret(&self, ctx: &Context<'_>) -> Result<String, Error> {
        let secret = KeyBuilder::new().generate().as_hex();

        let db = ctx.data::<DatabaseConnection>().unwrap();
        let session = ctx.data::<Session>().unwrap();

        let mut active_user: user::ActiveModel = session.user.clone().unwrap().into();
        active_user.tfa_secret = ActiveValue::Set(Some(secret.clone()));

        match active_user.update(db).await {
            Ok(_value) => match TOTPBuilder::new().hex_key(&secret.to_owned()).finalize() {
                Ok(totp) => Ok(totp
                    .key_uri_format("Starship", &session.user.clone().unwrap().username)
                    .finalize()),
                Err(error) => {
                    error!("{:?}", error);
                    Err(errors::create_internal_server_error(
                        None,
                        "TOTP_BUILD_ERROR",
                    ))
                }
            },
            Err(error) => {
                error!("{}", error);
                Err(errors::create_internal_server_error(None, "UPDATE_ERROR"))
            }
        }
    }
}
