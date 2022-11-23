#![allow(non_snake_case)]
use super::super::custom_emoji;
use super::super::planet;
use super::super::planet_member;
use super::super::user;
use super::super::user::Model;
use crate::errors;
use crate::sessions::Session;
use async_graphql::types::ID;
use async_graphql::{Context, Error, Object};
use chrono::NaiveDateTime;
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};

impl Model {
    fn user_id_is_same(&self, ctx: &Context<'_>, name: &str) -> Result<(), Error> {
        let session = ctx.data::<Session>();

        match session {
            Ok(session) => match session.user.clone() {
                Some(user) => {
                    if user.id == self.id || user.admin {
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

    #[graphql(deprecation = "following is deprecated in favor of the role system, use memberOf")]
    async fn following(&self, ctx: &Context<'_>) -> Result<Vec<planet::Model>, Error> {
        self.user_id_is_same(ctx, "following")?;

        Ok(vec![])
    }

    async fn memberOf(&self, ctx: &Context<'_>) -> Result<Vec<planet::Model>, Error> {
        self.user_id_is_same(ctx, "memberOf")?;

        let db = ctx.data::<DatabaseConnection>().unwrap();

        match planet_member::Entity::find()
            .filter(planet_member::Column::User.eq(self.id.clone()))
            .find_with_related(planet::Entity)
            .all(db)
            .await
        {
            Ok(members) => Ok(members
                .iter()
                .filter_map(|value| {
                    if !value.1.is_empty() {
                        Some(value.1[0].clone())
                    } else {
                        None
                    }
                })
                .collect()),
            Err(_error) => Err(errors::create_internal_server_error(
                None,
                "FIND_PLANETS_ERROR",
            )),
        }
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

    async fn blockedUsers(&self, ctx: &Context<'_>) -> Result<Vec<Model>, Error> {
        self.user_id_is_same(ctx, "blockedUsers")?;

        let db = ctx.data::<DatabaseConnection>().unwrap();

        match user::Entity::find()
            .filter(user::Column::Id.is_in(self.blocked.clone()))
            .all(db)
            .await
        {
            Ok(value) => Ok(value),
            Err(_error) => Err(errors::create_internal_server_error(
                None,
                "FIND_BLOCKED_ERROR",
            )),
        }
    }

    async fn customEmojis(&self, ctx: &Context<'_>) -> Result<Vec<custom_emoji::Model>, Error> {
        let db = ctx.data::<DatabaseConnection>().unwrap();

        match custom_emoji::Entity::find()
            .filter(
                custom_emoji::Column::Owner
                    .eq(self.id.clone())
                    .and(custom_emoji::Column::Planet.is_null()),
            )
            .all(db)
            .await
        {
            Ok(value) => Ok(value),
            Err(_error) => Err(errors::create_internal_server_error(
                None,
                "FIND_EMOJIS_ERROR",
            )),
        }
    }

    async fn online(&self) -> bool {
        !self.sessions.is_empty()
    }

    async fn notificationSetting(&self, ctx: &Context<'_>) -> Result<i16, Error> {
        self.user_id_is_same(ctx, "notificationSetting")?;

        Ok(self.notification_setting)
    }
}
