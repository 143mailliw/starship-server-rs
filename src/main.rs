mod db;
mod entities;

use actix_web::{guard, web, App, HttpResponse, HttpServer};
use async_graphql::{http::GraphiQLSource, EmptyMutation, EmptySubscription, Schema, Context};
use async_graphql_actix_web::{GraphQLRequest, GraphQLResponse};
use db::set_up_db;
use dotenv;
use sea_orm::{ConnectionTrait, DatabaseConnection, DbBackend};
use std::io::Result;

struct Query;

#[async_graphql::Object]
impl Query {
    async fn howdy(&self, ctx: &Context<'_>) -> &str {
        let db = ctx.data::<DatabaseConnection>().unwrap();
        match db.get_database_backend() {
            DbBackend::Postgres => "Correct",
            _ => "Wrong"
        }
    }
}

async fn index(
    schema: web::Data<Schema<Query, EmptyMutation, EmptySubscription>>,
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
    dotenv::dotenv().ok();

    let db = match set_up_db().await {
        Ok(db) => db,
        Err(err) => panic!("fatal: {} ", err)
    };

    let schema = Schema::build(Query, EmptyMutation, EmptySubscription)
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
