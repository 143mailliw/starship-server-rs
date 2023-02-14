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
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, ModelTrait, QueryFilter};

impl Model {
    fn user_id_is_same(&self, ctx: &Context<'_>, name: &str) -> Result<(), Error> {
        let session = ctx.data::<Session>();

        match session {
            Ok(session) => match session.user.as_ref() {
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

#[Object(name = "User", rename_fields = "camelCase", rename_args = "camelCase")]
impl Model {
    #[graphql(complexity = 0)]
    async fn id(&self) -> ID {
        ID(self.id.clone())
    }

    #[graphql(complexity = 0)]
    async fn username(&self) -> String {
        self.username.clone()
    }

    #[graphql(complexity = 0)]
    async fn admin(&self) -> bool {
        self.admin
    }

    #[graphql(complexity = 0)]
    async fn profile_picture(&self) -> Option<String> {
        self.profile_picture.clone()
    }

    #[graphql(complexity = 0)]
    async fn profile_banner(&self) -> Option<String> {
        self.profile_banner.clone()
    }

    #[graphql(complexity = 0)]
    async fn profile_bio(&self) -> Option<String> {
        self.profile_bio.clone()
    }

    #[graphql(complexity = 0)]
    async fn banned(&self) -> bool {
        self.banned
    }

    #[graphql(complexity = 5)]
    async fn member_of(&self, ctx: &Context<'_>) -> Result<Vec<planet::Model>, Error> {
        self.user_id_is_same(ctx, "memberOf")?;

        let db = ctx.data::<DatabaseConnection>().unwrap();

        match self
            .find_related(planet_member::Entity)
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

    #[graphql(complexity = 0)]
    async fn created_at(&self) -> NaiveDateTime {
        self.created
    }

    #[graphql(complexity = 0)]
    async fn used_bytes(&self, ctx: &Context<'_>) -> Result<f64, Error> {
        self.user_id_is_same(ctx, "usedBytes")?;

        Ok(self.bytes_used as f64)
    }

    #[graphql(complexity = 0)]
    async fn cap_waived(&self, ctx: &Context<'_>) -> Result<bool, Error> {
        self.user_id_is_same(ctx, "capWaived")?;

        Ok(self.cap_waived)
    }

    #[graphql(complexity = 0)]
    async fn tfa_enabled(&self, ctx: &Context<'_>) -> Result<bool, Error> {
        self.user_id_is_same(ctx, "tfaEnabled")?;

        Ok(self.tfa_enabled)
    }

    #[graphql(complexity = 5)]
    async fn blocked_users(&self, ctx: &Context<'_>) -> Result<Vec<Model>, Error> {
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

    #[graphql(complexity = 5)]
    async fn custom_emojis(&self, ctx: &Context<'_>) -> Result<Vec<custom_emoji::Model>, Error> {
        let db = ctx.data::<DatabaseConnection>().unwrap();

        match self
            .find_related(custom_emoji::Entity)
            .filter(custom_emoji::Column::Planet.is_null())
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

    #[graphql(complexity = 0)]
    async fn online(&self) -> bool {
        !self.sessions.is_empty()
    }

    #[graphql(complexity = 0)]
    async fn notification_setting(&self, ctx: &Context<'_>) -> Result<i16, Error> {
        self.user_id_is_same(ctx, "notificationSetting")?;

        Ok(self.notification_setting)
    }
}
