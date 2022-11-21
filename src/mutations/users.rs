#![allow(non_snake_case)]
use crate::entities::prelude::Token;
use crate::entities::prelude::User;
use crate::entities::token;
use crate::entities::user;
use crate::errors;
use crate::sessions::Session;
use async_graphql::{Context, Error, Object, SimpleObject};
use bcrypt::hash;
use email_address::EmailAddress;
use log::error;
use nanoid::nanoid;
use sea_orm::{ActiveValue, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};
use std::env;

#[derive(SimpleObject)]
struct LoginPayload {
    token: String,
    expectingTFA: bool,
}

#[Object]
impl super::Mutation {
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
            following: ActiveValue::Set(vec![]),
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
            user: ActiveValue::Set(user.id),
            ip: ActiveValue::Set(addr),
            location: ActiveValue::Set("Unknown".to_string()),
            latitude: ActiveValue::Set(None),
            longitude: ActiveValue::Set(None),
            browser: ActiveValue::Set("Unknown".to_string()),
            operating_system: ActiveValue::Set("Unknown".to_string()),
        };

        match Token::insert(token).exec(db).await {
            Ok(res) => Ok(LoginPayload {
                token: res.last_insert_id,
                expectingTFA: user.tfa_enabled,
            }),
            Err(error) => {
                error!("{}", error);
                Err(errors::create_internal_server_error(
                    None,
                    "INSERTION_ERROR",
                ))
            }
        }
    }
}
