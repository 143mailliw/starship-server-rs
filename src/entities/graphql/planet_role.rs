use super::super::planet;
use super::super::planet_member;
use super::super::planet_role::Model;
use crate::errors;
use async_graphql::types::ID;
use async_graphql::{Context, Error, Object};
use sea_orm::{
    DatabaseBackend, DatabaseConnection, EntityTrait, ModelTrait, PaginatorTrait, Statement,
};

#[Object(
    name = "PlanetRole",
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
    async fn color(&self) -> &String {
        &self.color
    }

    #[graphql(complexity = 0)]
    async fn permissions(&self) -> &Vec<String> {
        &self.permissions
    }

    #[graphql(complexity = 5)]
    async fn planet(&self, ctx: &Context<'_>) -> Result<planet::Model, Error> {
        let db = ctx.data::<DatabaseConnection>().unwrap();

        self.find_related(planet::Entity)
            .one(db)
            .await
            .map_err(|_| errors::create_internal_server_error(None, "FIND_PLANET_ERROR"))?
            .ok_or(errors::create_internal_server_error(
                None,
                "PLANET_MISSING_ERROR",
            ))
    }

    #[graphql(complexity = 0)]
    async fn position(&self) -> i32 {
        self.position
    }

    #[graphql(complexity = 0)]
    async fn default(&self) -> bool {
        self.default
    }

    #[graphql(complexity = "5 * size as usize + size as usize * child_complexity")]
    async fn members(
        &self,
        ctx: &Context<'_>,
        size: u64,
        page: u64,
    ) -> Result<Vec<planet_member::Model>, Error> {
        let db = ctx.data::<DatabaseConnection>().unwrap();

        planet_member::Entity::find()
            .from_raw_sql(Statement::from_sql_and_values(
                DatabaseBackend::Postgres,
                r#"SELECT * FROM planet_member WHERE $1=ANY(roles) ORDER BY planet_member.created DESC"#,
                [self.id.clone().into()],
            ))
            .paginate(db, size)
            .fetch_page(page)
            .await
            .map_err(|_| errors::create_internal_server_error(None, "FIND_MEMBERS_ERROR"))
    }
}
