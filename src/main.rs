mod entities;
mod types;

use std::io::Result;
use actix_web::{guard, web, App, HttpResponse, HttpServer};
use async_graphql::{Schema, EmptyMutation, EmptySubscription, http::GraphiQLSource};
use async_graphql_actix_web::{GraphQLRequest, GraphQLResponse};

struct Query;

#[async_graphql::Object]
impl Query {
  async fn howdy(&self) -> &'static str {
    "partner"
  }
}

async fn index(
    schema: web::Data<Schema<Query, EmptyMutation, EmptySubscription>>,
    req: GraphQLRequest
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

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(Schema::new(
                Query,
                EmptyMutation,
                EmptySubscription
            )))
            .service(web::resource("/").guard(guard::Post()).to(index))
            .service(web::resource("/").guard(guard::Get()).to(gql_playgound))
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
}
