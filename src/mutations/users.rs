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
use libreauth::oath::TOTPBuilder;
use nanoid::nanoid;
use rand::prelude::*;
use sea_orm::{
    ActiveModelTrait, ActiveValue, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter,
};
use sha2::Sha256;
use std::env;

#[derive(SimpleObject)]
struct LoginPayload {
    token: String,
    #[graphql(name = "expectingTfa")]
    expecting_tfa: bool,
}

#[derive(Default, Description)]
pub struct UserMutation;

#[Object(rename_fields = "camelCase", rename_args = "camelCase")]
impl UserMutation {
    /// Registers a new user.
    #[graphql(complexity = 200)]
    async fn insert_user(
        &self,
        ctx: &Context<'_>,
        email: String,
        password: String,
        username: String,
        _recaptcha: String,
    ) -> Result<user::Model, Error> {
        // TODO: add RECAPTCHA support
        // TODO: verify email

        let db = ctx.data::<DatabaseConnection>().unwrap();

        let existing_user = User::find()
            .filter(
                user::Column::Username
                    .eq(username.clone())
                    .or(user::Column::EmailAddress.eq(email.clone())),
            )
            .one(db)
            .await
            .map_err(|_| errors::create_internal_server_error(None, "FIND_ERROR"))?;

        if let Some(existing_user) = existing_user {
            if existing_user.email_address == email {
                return Err(errors::create_user_input_error(
                    "An account is already registered with that email.",
                    "EMAIL_ALREADY_EXISTS",
                ));
            }

            if existing_user.username == username {
                return Err(errors::create_user_input_error(
                    "An account is already registered with that username",
                    "USERNAME_ALREADY_EXISTS",
                ));
            }
        }

        if username.len() < 4 {
            return Err(errors::create_user_input_error(
                "Your username must be at least 4 characters.",
                "USERNAME_TOO_SHORT",
            ));
        };

        if !EmailAddress::is_valid(&email) {
            return Err(errors::create_user_input_error(
                "Invalid email address.",
                "INVALID_EMAIL_ADDRESS",
            ));
        };

        // inputs are valid
        let hash = hash(password, 4)
            .map_err(|_| errors::create_internal_server_error(None, "HASH_ERROR"))?;

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

        let result = User::insert(user)
            .exec(db)
            .await
            .map_err(|_| errors::create_internal_server_error(None, "INSERTION_ERROR"))?;

        User::find_by_id(result.last_insert_id)
            .one(db)
            .await
            .map_err(|_| errors::create_internal_server_error(None, "RETRIEVAL_ERROR"))?
            .ok_or(errors::create_internal_server_error(None, "BAD_ID_ERROR"))
    }

    /// Creates a new token & and signs a JWT object containing it's ID.
    #[graphql(complexity = 200)]
    async fn login_user(
        &self,
        ctx: &Context<'_>,
        username: String,
        password: String,
    ) -> Result<LoginPayload, Error> {
        let db = ctx.data::<DatabaseConnection>().unwrap();
        let session = ctx.data::<Session>().unwrap();

        let user = User::find()
            .filter(user::Column::Username.eq(username))
            .one(db)
            .await
            .map_err(|_| errors::create_internal_server_error(None, "FIND_ERROR"))?
            .ok_or(errors::create_forbidden_error(
                Some("Invalid username or password."),
                "INVALID_USER",
            ))?;

        if env::var("SMTP_HOST").is_ok() && !user.verified {
            return Err(errors::create_forbidden_error(
                Some("You need to verify your email."),
                "UNVERIFIED_EMAIL",
            ));
        };

        let result = bcrypt::verify(password, &user.password)
            .map_err(|_| errors::create_internal_server_error(None, "VERIFICATION_ERROR"))?;

        if !result {
            return Err(errors::create_forbidden_error(
                Some("Invalid username or password."),
                "INVALID_USER",
            ));
        }

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
            verified: ActiveValue::Set(!user.tfa_enabled),
        };

        let res = Token::insert(token)
            .exec(db)
            .await
            .map_err(|_| errors::create_internal_server_error(None, "INSERTION_ERROR"))?;

        let jwt_data = JWTLoginToken {
            token: res.last_insert_id,
            user_id: user.id,
        };

        let secret = env::var("SECRET").unwrap();
        let key: Hmac<Sha256> = Hmac::new_from_slice(secret.as_bytes()).unwrap();
        let token = jwt_data.sign_with_key(&key).unwrap();

        Ok(LoginPayload {
            token,
            expecting_tfa: user.tfa_enabled,
        })
    }

    /// Toggles whether or not a user is banned.
    #[graphql(guard = "SessionGuard::new(SessionType::Admin)", complexity = 10)]
    async fn ban_user(&self, ctx: &Context<'_>, user_id: ID) -> Result<user::Model, Error> {
        let db = ctx.data::<DatabaseConnection>().unwrap();
        let id = user_id.to_string();

        let user = User::find_by_id(id)
            .one(db)
            .await
            .map_err(|_| errors::create_internal_server_error(None, "RETRIEVAL_ERROR"))?
            .ok_or(errors::create_not_found_error())?;

        if user.admin {
            return Err(errors::create_forbidden_error(
                Some("You cannot ban an administrator."),
                "ADMINISTRATIVE_IMMUNITY",
            ));
        }

        let mut active_user: user::ActiveModel = user.clone().into();
        active_user.banned = ActiveValue::Set(!user.banned);

        active_user
            .update(db)
            .await
            .map_err(|_| errors::create_internal_server_error(None, "UPDATE_ERROR"))
    }

    /// Toggles whether or not a user is blocked.
    #[graphql(guard = "SessionGuard::new(SessionType::User)", complexity = 10)]
    async fn toggle_block_user(
        &self,
        ctx: &Context<'_>,
        user_id: ID,
    ) -> Result<user::Model, Error> {
        let db = ctx.data::<DatabaseConnection>().unwrap();
        let session = ctx.data::<Session>().unwrap();
        let id = user_id.to_string();

        let user = User::find_by_id(id)
            .one(db)
            .await
            .map_err(|_| errors::create_internal_server_error(None, "RETRIEVAL_ERROR"))?
            .ok_or(errors::create_not_found_error())?;

        if user.id == session.user.as_ref().unwrap().id {
            return Err(errors::create_user_input_error(
                "You cannot block yourself.",
                "INVALID_USER_SELF",
            ));
        }

        let mut blocked = session.user.as_ref().unwrap().blocked.clone();
        if blocked.contains(&user.id) {
            blocked.retain(|searched_user| **searched_user != user.id);
        } else {
            blocked.push(user.id);
        };

        let mut active_user: user::ActiveModel = session.user.clone().unwrap().into();
        active_user.blocked = ActiveValue::Set(blocked);

        active_user
            .update(db)
            .await
            .map_err(|_| errors::create_internal_server_error(None, "UPDATE_ERROR"))
    }

    /// Changes the current user's profile bio.
    #[graphql(guard = "SessionGuard::new(SessionType::NotBanned)", complexity = 10)]
    async fn update_profile_bio(
        &self,
        ctx: &Context<'_>,
        bio: String,
    ) -> Result<user::Model, Error> {
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

        active_user
            .update(db)
            .await
            .map_err(|_| errors::create_internal_server_error(None, "UPDATE_ERROR"))
    }

    /// Changes the current user's notification setting.
    #[graphql(guard = "SessionGuard::new(SessionType::User)", complexity = 10)]
    async fn set_notification_setting(
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

        active_user
            .update(db)
            .await
            .map_err(|_| errors::create_internal_server_error(None, "UPDATE_ERROR"))
    }

    /// Generates and stores a new TOTP secret for the current user.
    /// NB: This is step 1/2 in the TOTP flow. You need to run the `confirmTFA` mutation as well.
    #[graphql(guard = "SessionGuard::new(SessionType::User)", complexity = 10)]
    async fn generate_totp_secret(&self, ctx: &Context<'_>) -> Result<String, Error> {
        let secret = KeyBuilder::new().generate().as_hex();

        let db = ctx.data::<DatabaseConnection>().unwrap();
        let session = ctx.data::<Session>().unwrap();

        let mut active_user: user::ActiveModel = session.user.clone().unwrap().into();
        active_user.tfa_secret = ActiveValue::Set(Some(secret.clone()));

        active_user
            .update(db)
            .await
            .map_err(|_| errors::create_internal_server_error(None, "UPDATE_ERROR"))?;

        Ok(TOTPBuilder::new()
            .hex_key(&secret)
            .finalize()
            .map_err(|_| errors::create_internal_server_error(None, "TOTP_BUILD_ERROR"))?
            .key_uri_format("Starship", &session.user.as_ref().unwrap().username)
            .finalize())
    }

    /// Validates the token, and, if the token is valid, enables 2FA and generates backup codes.
    #[graphql(guard = "SessionGuard::new(SessionType::User)", complexity = 10)]
    async fn confirm_tfa(&self, ctx: &Context<'_>, token: u32) -> Result<Vec<u32>, Error> {
        let db = ctx.data::<DatabaseConnection>().unwrap();
        let session = ctx.data::<Session>().unwrap();

        let user = session.user.as_ref().unwrap();

        if user.tfa_secret.is_none() {
            return Err(errors::create_user_input_error(
                "You must generate the TOTP secret before confirming it.",
                "NO_SECRET",
            ));
        };

        if user.tfa_enabled {
            return Err(errors::create_user_input_error(
                "Two factor authentication is already enabled & confirmed.",
                "TFA_ALREADY_ENABLED",
            ));
        };

        let is_valid = TOTPBuilder::new()
            .hex_key(user.tfa_secret.as_ref().unwrap())
            .finalize()
            .map_err(|_| errors::create_internal_server_error(None, "TOTP_BUILD_ERROR"))?
            .is_valid(&token.to_string());

        if is_valid {
            let mut rng = StdRng::from_entropy();
            let numbers = vec![
                rng.gen_range(0..1_000_000_000),
                rng.gen_range(0..1_000_000_000),
                rng.gen_range(0..1_000_000_000),
                rng.gen_range(0..1_000_000_000),
                rng.gen_range(0..1_000_000_000),
                rng.gen_range(0..1_000_000_000),
                rng.gen_range(0..1_000_000_000),
                rng.gen_range(0..1_000_000_000),
            ];

            let mut active_user: user::ActiveModel = session.user.clone().unwrap().into();
            active_user.tfa_backup = ActiveValue::Set(
                numbers
                    .iter()
                    .map(std::string::ToString::to_string)
                    .collect(),
            );
            active_user.tfa_enabled = ActiveValue::Set(true);

            active_user
                .update(db)
                .await
                .map_err(|_| errors::create_internal_server_error(None, "UPDATE_ERROR"))?;

            Ok(numbers)
        } else {
            Err(errors::create_user_input_error(
                "Incorrect TFA code.",
                "INCORRECT_CODE",
            ))
        }
    }

    /// Validates the token, and, if the token is valid, disables TFA for the current user.
    #[graphql(guard = "SessionGuard::new(SessionType::User)", complexity = 10)]
    async fn disable_tfa(&self, ctx: &Context<'_>, token: u32) -> Result<user::Model, Error> {
        let db = ctx.data::<DatabaseConnection>().unwrap();
        let session = ctx.data::<Session>().unwrap();

        let user = session.user.clone().unwrap();

        if !user.tfa_enabled {
            return Err(errors::create_user_input_error(
                "Two factor authentication is already disabled.",
                "TFA_ALREADY_DISABLED",
            ));
        };

        let is_valid = TOTPBuilder::new()
            .hex_key(&user.tfa_secret.unwrap())
            .finalize()
            .map_err(|_| errors::create_internal_server_error(None, "TOTP_BUILD_ERROR"))?
            .is_valid(&token.to_string());

        if is_valid || user.tfa_backup.contains(&token.to_string()) {
            let mut active_user: user::ActiveModel = session.user.clone().unwrap().into();
            active_user.tfa_enabled = ActiveValue::Set(false);

            active_user
                .update(db)
                .await
                .map_err(|_| errors::create_internal_server_error(None, "UPDATE_ERROR"))
        } else {
            Err(errors::create_user_input_error(
                "Incorrect TFA or backup code.",
                "INCORRECT_CODE",
            ))
        }
    }

    /// Verifies the authenticity of the current token, provided in the Authorization header.
    /// This mutation is only required if the user has TFA enabled.
    #[graphql(guard = "SessionGuard::new(SessionType::Token)", complexity = 200)]
    async fn finalize_authorization(&self, ctx: &Context<'_>, token: u32) -> Result<bool, Error> {
        let db = ctx.data::<DatabaseConnection>().unwrap();
        let session = ctx.data::<Session>().unwrap();

        let user = session.user.clone().unwrap();
        let auth_token = session.token.as_ref().unwrap();

        if !user.tfa_enabled {
            return Err(errors::create_user_input_error(
                "This user does not have two-factor authentication enabled.",
                "TFA_DISABLED",
            ));
        };

        if auth_token.verified {
            return Ok(true);
        }

        let is_valid = TOTPBuilder::new()
            .hex_key(user.tfa_secret.as_ref().unwrap())
            .finalize()
            .map_err(|_| errors::create_internal_server_error(None, "TOTP_BUILD_ERROR"))?
            .is_valid(&token.to_string());

        if is_valid || user.tfa_backup.contains(&token.to_string()) {
            if user.tfa_backup.contains(&token.to_string()) {
                let mut remaining_codes = user.tfa_backup.clone();
                remaining_codes.retain(|searched_code| searched_code != &token.to_string());

                let mut active_user: user::ActiveModel = user.into();
                active_user.tfa_backup = ActiveValue::Set(remaining_codes);

                active_user
                    .update(db)
                    .await
                    .map_err(|_| errors::create_internal_server_error(None, "UPDATE_USER_ERROR"))?;
            }

            let mut active_token: token::ActiveModel = auth_token.clone().into();
            active_token.verified = ActiveValue::Set(true);

            active_token
                .update(db)
                .await
                .map_err(|_| errors::create_internal_server_error(None, "UPDATE_TOKEN_ERROR"))
                .map(|_| true)
        } else {
            Err(errors::create_user_input_error(
                "Incorrect TFA or backup code.",
                "INCORRECT_CODE",
            ))
        }
    }
}
