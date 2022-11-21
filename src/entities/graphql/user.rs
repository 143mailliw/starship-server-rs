#![allow(non_snake_case)]
use super::super::user::Model;
use crate::errors;
use crate::sessions::Session;
use async_graphql::types::ID;
use async_graphql::{Context, Error, Object};
use chrono::NaiveDateTime;

impl Model {
    fn user_id_is_same(&self, ctx: &Context<'_>, name: &str) -> Result<(), Error> {
        let session = ctx.data::<Session>();

        match session {
            Ok(session) => match session.user.clone() {
                Some(user) => {
                    if user.id == self.id {
                        Ok(())
                    } else {
                        Err(errors::create_forbidden_error(
                            Some(
                                ("You don't have permission to read the field '".to_string()
                                    + name
                                    + "'.")
                                    .as_str(),
                            ),
                            "FORBIDDEN",
                        ))
                    }
                }
                None => Err(errors::create_forbidden_error(
                    Some(
                        ("You don't have permission to read the field '".to_string() + name + "'.")
                            .as_str(),
                    ),
                    "FORBIDDEN",
                )),
            },
            Err(_error) => Err(errors::create_internal_server_error(
                None,
                "NO_SESSION_ERROR",
            )),
        }
    }
}

#[Object(name = "User")]
impl Model {
    async fn id(&self) -> ID {
        ID(self.id.clone())
    }

    async fn username(&self) -> String {
        self.username.clone()
    }

    async fn admin(&self) -> bool {
        self.admin
    }

    async fn profilePicture(&self) -> Option<String> {
        self.profile_picture.clone()
    }

    async fn profileBanner(&self) -> Option<String> {
        self.profile_banner.clone()
    }

    async fn profileBio(&self) -> Option<String> {
        self.profile_bio.clone()
    }

    async fn banned(&self) -> bool {
        self.banned
    }

    async fn following(&self, ctx: &Context<'_>) -> Result<Vec<String>, Error> {
        // TODO: return a vec of Planets instead
        self.user_id_is_same(ctx, "following")?;

        Ok(self.following.clone())
    }

    // this function is obsolete
    #[graphql(deprecation = "memberOf is deprecated in favor of the role system, use following")]
    async fn memberOf(&self, ctx: &Context<'_>) -> Result<Vec<String>, Error> {
        self.user_id_is_same(ctx, "memberOf")?;

        // TODO: return a vec of Planets instead
        Ok(vec![])
    }

    async fn createdAt(&self) -> NaiveDateTime {
        self.created
    }

    async fn usedBytes(&self, ctx: &Context<'_>) -> Result<f64, Error> {
        self.user_id_is_same(ctx, "usedBytes")?;

        Ok(self.bytes_used as f64)
    }

    async fn capWaived(&self, ctx: &Context<'_>) -> Result<bool, Error> {
        self.user_id_is_same(ctx, "capWaived")?;

        Ok(self.cap_waived)
    }

    async fn tfaEnabled(&self, ctx: &Context<'_>) -> Result<bool, Error> {
        self.user_id_is_same(ctx, "tfaEnabled")?;

        Ok(self.tfa_enabled)
    }

    async fn blockedUsers(&self, ctx: &Context<'_>) -> Result<Vec<String>, Error> {
        // TODO: return a vec of Users instead
        self.user_id_is_same(ctx, "blockedUsers")?;

        Ok(self.blocked.clone())
    }

    async fn online(&self) -> bool {
        !self.sessions.is_empty()
    }

    async fn notificationSetting(&self, ctx: &Context<'_>) -> Result<i16, Error> {
        self.user_id_is_same(ctx, "notificationSetting")?;

        Ok(self.notification_setting)
    }
}
