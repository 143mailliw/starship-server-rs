mod db;
mod entities;
mod errors;
mod guards;
mod mutations;
mod queries;
mod sessions;

use actix_cors::Cors;
use actix_web::{guard, web, App, HttpRequest, HttpResponse, HttpServer};
use async_graphql::{http::GraphiQLSource, EmptySubscription, Schema};
use async_graphql_actix_web::{GraphQLRequest, GraphQLResponse};
use db::set_up_db;
use sea_orm::DatabaseConnection;
use std::io::Result;

async fn index(
    schema: web::Data<Schema<queries::Query, mutations::Mutation, EmptySubscription>>,
    db: web::Data<DatabaseConnection>,
    req: HttpRequest,
    gql_req: GraphQLRequest,
) -> GraphQLResponse {
    let mut request = gql_req.into_inner();
    request = request
        .data(sessions::Session::make_session_from_request(&req, (*db.into_inner()).clone()).await);
    schema.execute(request).await.into()
}

async fn gql_schema(
    schema: web::Data<Schema<queries::Query, mutations::Mutation, EmptySubscription>>,
) -> HttpResponse {
    HttpResponse::Ok()
        .content_type("text/plain; charset=utf-8")
        .body(schema.sdl())
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

    let schema = Schema::build(
        queries::Query::default(),
        mutations::Mutation::default(),
        EmptySubscription,
    )
    .data(db.clone())
    .finish();

    HttpServer::new(move || {
        let cors = Cors::permissive();

        App::new()
            .wrap(cors)
            .app_data(web::Data::new(schema.clone()))
            .app_data(web::Data::new(db.clone()))
            .service(web::resource("/graphql").guard(guard::Post()).to(index))
            .service(web::resource("/schema").guard(guard::Get()).to(gql_schema))
            .service(
                web::resource("/graphql")
                    .guard(guard::Get())
                    .to(gql_playgound),
            )
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
}
