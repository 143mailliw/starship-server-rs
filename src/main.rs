mod db;
mod entities;
mod errors;
mod mutations;

use actix_web::{guard, web, App, HttpResponse, HttpServer};
use async_graphql::{http::GraphiQLSource, Context, EmptyMutation, EmptySubscription, Schema};
use async_graphql_actix_web::{GraphQLRequest, GraphQLResponse};
use db::set_up_db;
use sea_orm::{ConnectionTrait, DatabaseConnection, DbBackend};
use std::io::Result;

struct Query;

#[async_graphql::Object]
impl Query {
    async fn howdy(&self, ctx: &Context<'_>) -> &str {
        let db = ctx.data::<DatabaseConnection>().unwrap();
        match db.get_database_backend() {
            DbBackend::Postgres => "Correct",
            _ => "Wrong",
        }
    }
}

async fn index(
    schema: web::Data<Schema<Query, mutations::Mutation, EmptySubscription>>,
    req: GraphQLRequest,
) -> GraphQLResponse {
    schema.execute(req.into_inner()).await.into()
}

async fn gql_playgound() -> HttpResponse {
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(
            GraphiQLSource::build()
                //.endpoint("http://localhost:8000")
                .finish(),
        )
}

#[actix_web::main]
async fn main() -> Result<()> {
    env_logger::init();

    dotenv::dotenv().ok();

    let db = match set_up_db().await {
        Ok(db) => db,
        Err(err) => panic!("fatal: {} ", err),
    };

    let schema = Schema::build(Query, mutations::Mutation, EmptySubscription)
        .data(db)
        .finish();

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(schema.clone()))
            .service(web::resource("/").guard(guard::Post()).to(index))
            .service(web::resource("/").guard(guard::Get()).to(gql_playgound))
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
}
