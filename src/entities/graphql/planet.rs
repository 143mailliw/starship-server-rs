use super::super::custom_emoji;
use super::super::planet::Model;
use super::super::planet_component;
use super::super::planet_member;
use super::super::planet_role;
use super::super::user;
use crate::errors;
use async_graphql::types::ID;
use async_graphql::{Context, Error, Object};
use chrono::NaiveDateTime;
use sea_orm::{DatabaseConnection, EntityTrait, ModelTrait, PaginatorTrait, QueryOrder};

#[Object(
    name = "Planet",
    rename_fields = "camelCase",
    rename_args = "camelCase"
)]
impl Model {
    #[graphql(complexity = 0)]
    async fn id(&self) -> ID {
        ID(self.id.clone())
    }

    #[graphql(complexity = 0)]
    async fn name(&self) -> &String {
        &self.name
    }

    #[graphql(complexity = 0)]
    async fn created_at(&self) -> NaiveDateTime {
        self.created
    }

    #[graphql(complexity = 5)]
    async fn owner(&self, ctx: &Context<'_>) -> Result<user::Model, Error> {
        let db = ctx.data::<DatabaseConnection>().unwrap();

        user::Entity::find_by_id(self.owner.clone())
            .one(db)
            .await
            .map_err(|_| errors::create_internal_server_error(None, "FIND_OWNER_ERROR"))?
            .ok_or(errors::create_internal_server_error(
                None,
                "OWNER_MISSING_ERROR",
            ))
    }

    #[graphql(complexity = 0)]
    async fn private(&self) -> bool {
        self.private
    }

    #[graphql(complexity = 0)]
    async fn member_count(&self) -> i32 {
        self.member_count
    }

    #[graphql(complexity = 5)]
    async fn components(&self, ctx: &Context<'_>) -> Result<Vec<planet_component::Model>, Error> {
        let db = ctx.data::<DatabaseConnection>().unwrap();

        self.find_related(planet_component::Entity)
            .all(db)
            .await
            .map_err(|_| errors::create_internal_server_error(None, "FIND_COMPONENTS_ERROR"))
    }

    #[graphql(complexity = 5)]
    async fn home_component(
        &self,
        ctx: &Context<'_>,
    ) -> Result<Option<planet_component::Model>, Error> {
        let db = ctx.data::<DatabaseConnection>().unwrap();

        match &self.home {
            Some(id) => planet_component::Entity::find_by_id(id.clone())
                .one(db)
                .await
                .map_err(|_| {
                    errors::create_internal_server_error(None, "FIND_HOME_COMPONENT_ERROR")
                }),
            None => Ok(None),
        }
    }

    #[graphql(complexity = 0)]
    async fn featured(&self) -> bool {
        self.featured
    }

    #[graphql(complexity = 0)]
    async fn verified(&self) -> bool {
        self.verified
    }

    #[graphql(complexity = 0)]
    async fn partnered(&self) -> bool {
        self.partnered
    }

    #[graphql(complexity = "5 * size as usize + size as usize * child_complexity")]
    async fn members(
        &self,
        ctx: &Context<'_>,
        size: u64,
        page: u64,
    ) -> Result<Vec<planet_member::Model>, Error> {
        let db = ctx.data::<DatabaseConnection>().unwrap();

        self.find_related(planet_member::Entity)
            .order_by_desc(planet_member::Column::Created)
            .paginate(db, size)
            .fetch_page(page)
            .await
            .map_err(|_| errors::create_internal_server_error(None, "FIND_MEMBERS_ERROR"))
    }

    #[graphql(complexity = "5")]
    async fn roles(&self, ctx: &Context<'_>) -> Result<Vec<planet_role::Model>, Error> {
        let db = ctx.data::<DatabaseConnection>().unwrap();

        self.find_related(planet_role::Entity)
            .all(db)
            .await
            .map_err(|_| errors::create_internal_server_error(None, "FIND_ROLES_ERROR"))
    }

    #[graphql(complexity = 0)]
    async fn featured_description(&self) -> &String {
        &self.featured_description
    }

    // TODO: invites

    #[graphql(complexity = 0)]
    async fn css(&self) -> &String {
        &self.css
    }

    #[graphql(complexity = 0)]
    async fn description(&self) -> &Option<String> {
        &self.description
    }

    #[graphql(complexity = 5)]
    async fn custom_emojis(&self, ctx: &Context<'_>) -> Result<Vec<custom_emoji::Model>, Error> {
        let db = ctx.data::<DatabaseConnection>().unwrap();

        self.find_related(custom_emoji::Entity)
            .all(db)
            .await
            .map_err(|_| errors::create_internal_server_error(None, "FIND_EMOJIS_ERROR"))
    }

    // TODO: unread
    // TODO: mentioned
}
